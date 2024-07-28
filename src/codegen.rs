use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{CodeModel, FileType, RelocMode, Target, TargetTriple};
use inkwell::types::BasicType;
use inkwell::OptimizationLevel;
use std::path::Path;

use crate::ast::Function;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> CodeGen<'ctx> {
        let module = context.create_module(module_name);
        let codegen = CodeGen {
            context,
            module,
            builder: context.create_builder(),
        };

        return codegen;
    }
}

impl<'ctx> CodeGen<'ctx> {
    pub fn compile_fn(&self, f: Function, file: &str) {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        let function = self.module.add_function(&f.name, fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0).unwrap().into_int_value();
        let y = function.get_nth_param(1).unwrap().into_int_value();
        let sum = self.builder.build_int_add(x, y, "idk_man").unwrap();
        self.builder.build_return(Some(&sum)).unwrap();

        let opt = OptimizationLevel::Default;
        let reloc = RelocMode::Default;
        let model = CodeModel::Default;
        let target = Target::from_name("x86-64");
        println!("{:?}", target);
        let target = target.unwrap();
        let target_machine = target
            .create_target_machine(
                &TargetTriple::create("x86_64-pc-linux-gnu"),
                "x86-64",
                "+avx2",
                opt,
                reloc,
                model,
            )
            .unwrap();

        target_machine
            .write_to_file(&self.module, FileType::Assembly, Path::new(file))
            .unwrap();
    }
}
