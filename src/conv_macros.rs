macro_rules! define_conversions {
    {$(
        $type: ident,
        $num_type_from: ident,
        $num_type_to: ident,
        [$($field: ident),+];
    )*} => {
$(

impl From<[$num_type_from; 2]> for $type {
    fn from(a: [$num_type_from; 2]) -> $type {
        let [$($field),*] = a;
        $type { $($field: $field as $num_type_to),* }
    }
}

impl From<&[$num_type_from; 2]> for $type {
    fn from(a: &[$num_type_from; 2]) -> $type {
        let [$($field),*] = a;
        $type { $($field: *$field as $num_type_to),* }
    }
}

impl From<($num_type_from, $num_type_from)> for $type {
    fn from(a: ($num_type_from, $num_type_from)) -> $type {
        let ($($field),*) = a;
        $type { $($field: $field as $num_type_to),* }
    }
}

impl From<&($num_type_from, $num_type_from)> for $type {
    fn from(a: &($num_type_from, $num_type_from)) -> $type {
        let ($($field),*) = a;
        $type { $($field: *$field as $num_type_to),* }
    }
}

)*
    };
}