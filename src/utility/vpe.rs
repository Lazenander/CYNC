use std::ops::Add;

pub trait VPE<V>: Sized {
    type P;
    type E;

    fn get_value(&self) -> &V;
    fn get_position(&self) -> &Self::P;
    fn get_errors(&self) -> &Self::E;

    fn transfer_vpe_ownership(self) -> (V, Self::P, Self::E);

    fn set_value(&mut self, value: V);
    fn set_position(&mut self, position: Self::P);
    fn set_error(&mut self, error: Self::E);

    fn new(value: V, position: Self::P, error: Self::E) -> Self;

    fn lift<V1, M1, Vres, Mres, FV, FP, FE>(oprd: M1, lift_v: FV, lift_p: FP, lift_e: FE) -> Mres
    where
        M1: VPE<V1>,
        Mres: VPE<Vres>,
        FV: FnOnce(V1) -> Vres,
        FP: FnOnce(M1::P) -> Mres::P,
        FE: FnOnce(M1::E) -> Mres::E,
    {
        let (value, position, error) = oprd.transfer_vpe_ownership();
        Mres::new(lift_v(value), lift_p(position), lift_e(error))
    }

    fn merge<V1, M1, V2, M2, Vres, Mres, FV, FP, FE>(
        oprd1: M1,
        oprd2: M2,
        merge_v: FV,
        merge_p: FP,
        merge_e: FE,
    ) -> Mres
    where
        M1: VPE<V1>,
        M2: VPE<V2>,
        Mres: VPE<Vres>,
        FV: FnOnce(V1, V2) -> Vres,
        FP: FnOnce(M1::P, M2::P) -> Mres::P,
        FE: FnOnce(M1::E, M2::E) -> Mres::E,
    {
        let (value1, position1, error1) = oprd1.transfer_vpe_ownership();
        let (value2, position2, error2) = oprd2.transfer_vpe_ownership();
        Mres::new(
            merge_v(value1, value2),
            merge_p(position1, position2),
            merge_e(error1, error2),
        )
    }

    fn merge_3<V1, M1, V2, M2, V3, M3, Vres, Mres, FV, FP, FE>(
        oprd1: M1,
        oprd2: M2,
        oprd3: M3,
        merge_v: FV,
        merge_p: FP,
        merge_e: FE,
    ) -> Mres
    where
        M1: VPE<V1>,
        M2: VPE<V2>,
        M3: VPE<V3>,
        Mres: VPE<Vres>,
        FV: FnOnce(V1, V2, V3) -> Vres,
        FP: FnOnce(M1::P, M2::P, M3::P) -> Mres::P,
        FE: FnOnce(M1::E, M2::E, M3::E) -> Mres::E,
    {
        let (value1, position1, error1) = oprd1.transfer_vpe_ownership();
        let (value2, position2, error2) = oprd2.transfer_vpe_ownership();
        let (value3, position3, error3) = oprd3.transfer_vpe_ownership();
        Mres::new(
            merge_v(value1, value2, value3),
            merge_p(position1, position2, position3),
            merge_e(error1, error2, error3),
        )
    }

    fn lift_same_pe<V1, M1, Vres, Mres, F>(oprd: M1, lift: F) -> Mres
    where
        M1: VPE<V1>,
        Mres: VPE<Vres>,
        Mres::P: From<M1::P>,
        Mres::E: From<M1::E>,
        F: FnOnce(V1) -> Vres,
    {
        Self::lift(oprd, lift, Mres::P::from, Mres::E::from)
    }

    fn merge_same_pe<V1, M1, V2, M2, Vres, Mres, F>(oprd1: M1, oprd2: M2, merge: F) -> Mres
    where
        M1: VPE<V1>,
        M2: VPE<V2>,
        Mres: VPE<Vres>,
        F: FnOnce(V1, V2) -> Vres,
        Mres::P: From<M1::P>,
        Mres::P: From<M2::P>,
        Mres::E: From<M1::E>,
        Mres::E: From<M2::E>,
        Mres::P: Add<Output = Mres::P>,
        Mres::E: Add<Output = Mres::E>,
    {
        Self::merge(
            oprd1,
            oprd2,
            merge,
            |p1, p2| Mres::P::from(p1) + Mres::P::from(p2),
            |e1, e2| Mres::E::from(e1) + Mres::E::from(e2),
        )
    }

    fn merge_3_same_pe<V1, M1, V2, M2, V3, M3, Vres, Mres, F>(
        oprd1: M1,
        oprd2: M2,
        oprd3: M3,
        merge: F,
    ) -> Mres
    where
        M1: VPE<V1>,
        M2: VPE<V2>,
        M3: VPE<V3>,
        Mres: VPE<Vres>,
        F: FnOnce(V1, V2, V3) -> Vres,
        Mres::P: From<M1::P>,
        Mres::P: From<M2::P>,
        Mres::P: From<M3::P>,
        Mres::E: From<M1::E>,
        Mres::E: From<M2::E>,
        Mres::E: From<M3::E>,
        Mres::P: Add<Output = Mres::P>,
        Mres::E: Add<Output = Mres::E>,
    {
        Self::merge_3(
            oprd1,
            oprd2,
            oprd3,
            merge,
            |p1, p2, p3| Mres::P::from(p1) + Mres::P::from(p2) + Mres::P::from(p3),
            |e1, e2, e3| Mres::E::from(e1) + Mres::E::from(e2) + Mres::E::from(e3),
        )
    }
}
