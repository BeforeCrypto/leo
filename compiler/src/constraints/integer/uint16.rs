//! Methods to enforce constraints on uint16s in a resolved Leo program.

use crate::{
    constraints::{new_variable_from_variable, ConstrainedProgram, ConstrainedValue},
    types::{ParameterModel, ParameterValue, Variable},
    ConstrainedInteger,
};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{alloc::AllocGadget, eq::EqGadget, uint16::UInt16},
    },
};

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ConstrainedProgram<F, CS> {
    pub(crate) fn u16_from_parameter(
        &mut self,
        cs: &mut CS,
        scope: String,
        parameter_model: ParameterModel<F>,
        parameter_value: Option<ParameterValue<F>>,
    ) -> Variable<F> {
        // Check that the parameter value is the correct type
        let integer_option = parameter_value.map(|parameter| match parameter {
            ParameterValue::Integer(i) => i as u16,
            value => unimplemented!("expected integer parameter, got {}", value),
        });

        // Check visibility of parameter
        let name = parameter_model.variable.name.clone();
        let integer = if parameter_model.private {
            UInt16::alloc(cs.ns(|| name), || {
                integer_option.ok_or(SynthesisError::AssignmentMissing)
            })
            .unwrap()
        } else {
            UInt16::alloc_input(cs.ns(|| name), || {
                integer_option.ok_or(SynthesisError::AssignmentMissing)
            })
            .unwrap()
        };

        let parameter_variable = new_variable_from_variable(scope, &parameter_model.variable);

        // store each argument as variable in resolved program
        self.store_variable(
            parameter_variable.clone(),
            ConstrainedValue::Integer(ConstrainedInteger::U16(integer)),
        );

        parameter_variable
    }

    pub(crate) fn u16_array_from_parameter(
        &mut self,
        _cs: &mut CS,
        _scope: String,
        _parameter_model: ParameterModel<F>,
        _parameter_value: Option<ParameterValue<F>>,
    ) -> Variable<F> {
        unimplemented!("Cannot enforce integer array as parameter")
        // // Check visibility of parameter
        // let mut array_value = vec![];
        // let name = parameter.variable.name.clone();
        // for argument in argument_array {
        //     let number = if parameter.private {
        //         UInt16::alloc(cs.ns(|| name), Some(argument)).unwrap()
        //     } else {
        //         UInt16::alloc_input(cs.ns(|| name), Some(argument)).unwrap()
        //     };
        //
        //     array_value.push(number);
        // }
        //
        //
        // let parameter_variable = new_variable_from_variable(scope, &parameter.variable);
        //
        // // store array as variable in resolved program
        // self.store_variable(parameter_variable.clone(), ResolvedValue::U16Array(array_value));
        //
        // parameter_variable
    }

    pub(crate) fn enforce_u16_eq(cs: &mut CS, left: UInt16, right: UInt16) {
        left.enforce_equal(cs.ns(|| format!("enforce u16 equal")), &right)
            .unwrap();
    }

    pub(crate) fn enforce_u16_add(cs: &mut CS, left: UInt16, right: UInt16) -> UInt16 {
        UInt16::addmany(
            cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
            &[left, right],
        )
        .unwrap()
    }

    pub(crate) fn enforce_u16_sub(cs: &mut CS, left: UInt16, right: UInt16) -> UInt16 {
        left.sub(
            cs.ns(|| format!("enforce {} - {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )
        .unwrap()
    }

    pub(crate) fn enforce_u16_mul(cs: &mut CS, left: UInt16, right: UInt16) -> UInt16 {
        left.mul(
            cs.ns(|| format!("enforce {} * {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )
        .unwrap()
    }
    pub(crate) fn enforce_u16_div(cs: &mut CS, left: UInt16, right: UInt16) -> UInt16 {
        left.div(
            cs.ns(|| format!("enforce {} / {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )
        .unwrap()
    }
    pub(crate) fn enforce_u16_pow(cs: &mut CS, left: UInt16, right: UInt16) -> UInt16 {
        left.pow(
            cs.ns(|| {
                format!(
                    "enforce {} ** {}",
                    left.value.unwrap(),
                    right.value.unwrap()
                )
            }),
            &right,
        )
        .unwrap()
    }
}
