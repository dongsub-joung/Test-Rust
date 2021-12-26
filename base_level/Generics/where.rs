impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A,D> for YourType {}

impl <A, D> MyTrait<A,D> for YourType where 
    A: TraitB + TraitC,
    D: TraitE + TraitF {} 