use crate::export_class;

#[export_class(in_hicc)]
impl<T1, T2> (T1, T2) {
    fn field_0(&self) -> &T1 {
        &self.0
    }
    fn field_1(&self) -> &T2 {
        &self.1
    }
    fn take_0(self) -> T1 {
        self.0
    }
    fn take_1(self) -> T2 {
        self.1
    }
}

#[export_class(in_hicc)]
impl<T1, T2, T3> (T1, T2, T3) {
    fn field_0(&self) -> &T1 {
        &self.0
    }
    fn field_1(&self) -> &T2 {
        &self.1
    }
    fn field_2(&self) -> &T3 {
        &self.2
    }
    fn take_0(self) -> T1 {
        self.0
    }
    fn take_1(self) -> T2 {
        self.1
    }
    fn take_2(self) -> T3 {
        self.2
    }
}

#[export_class(in_hicc)]
impl<T1, T2, T3, T4> (T1, T2, T3, T4) {
    fn field_0(&self) -> &T1 {
        &self.0
    }
    fn field_1(&self) -> &T2 {
        &self.1
    }
    fn field_2(&self) -> &T3 {
        &self.2
    }
    fn field_3(&self) -> &T4 {
        &self.3
    }
    fn take_0(self) -> T1 {
        self.0
    }
    fn take_1(self) -> T2 {
        self.1
    }
    fn take_2(self) -> T3 {
        self.2
    }
    fn take_3(self) -> T4 {
        self.3
    }
}

#[export_class(in_hicc)]
impl<T1, T2, T3, T4, T5> (T1, T2, T3, T4, T5) {
    fn field_0(&self) -> &T1 {
        &self.0
    }
    fn field_1(&self) -> &T2 {
        &self.1
    }
    fn field_2(&self) -> &T3 {
        &self.2
    }
    fn field_3(&self) -> &T4 {
        &self.3
    }
    fn field_4(&self) -> &T5 {
        &self.4
    }
    fn take_0(self) -> T1 {
        self.0
    }
    fn take_1(self) -> T2 {
        self.1
    }
    fn take_2(self) -> T3 {
        self.2
    }
    fn take_3(self) -> T4 {
        self.3
    }
    fn take_4(self) -> T5 {
        self.4
    }
}

#[export_class(in_hicc)]
impl<T1, T2, T3, T4, T5, T6> (T1, T2, T3, T4, T5, T6) {
    fn field_0(&self) -> &T1 {
        &self.0
    }
    fn field_1(&self) -> &T2 {
        &self.1
    }
    fn field_2(&self) -> &T3 {
        &self.2
    }
    fn field_3(&self) -> &T4 {
        &self.3
    }
    fn field_4(&self) -> &T5 {
        &self.4
    }
    fn field_5(&self) -> &T6 {
        &self.5
    }
    fn take_0(self) -> T1 {
        self.0
    }
    fn take_1(self) -> T2 {
        self.1
    }
    fn take_2(self) -> T3 {
        self.2
    }
    fn take_3(self) -> T4 {
        self.3
    }
    fn take_4(self) -> T5 {
        self.4
    }
    fn take_5(self) -> T6 {
        self.5
    }
}
