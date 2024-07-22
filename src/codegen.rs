use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::types::BasicType;
use inkwell::OptimizationLevel;
use inkwell::targets::{CodeModel, RelocMode, FileType, Target, TargetMachine, TargetTriple, InitializationConfig};

use crate::ast::Function;


struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl <'ctx> CodeGen<'ctx> {
    fn compile_fn(&self, f: Function, file: &str) {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()],false);
        let function = self.module.add_function(&f.name, fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0).unwrap().into_int_value();
        let y = function.get_nth_param(1).unwrap().into_int_value();
        let sum = self.builder.build_int_add(x, y, "idk_man");
        self.builder.build_return(Some(&sum)).unwrap();

        /* let opt = OptimizationLevel::Default;
        let reloc = RelocMode::Default;
        let model = CodeModel::Default;
        let target = Target::from_name("x86-64").unwrap();
        let target_machine = target.create_target_machine(
            &TargetTriple::create("x86_64-pc-linux-gnu"),
            "x86-64",
            "+avx2",
            opt,
            reloc,
            model
        )
        .unwrap();

        target_machine.write_to_file(&self.module, FileType::Assembly, file); */
        
    }
}
