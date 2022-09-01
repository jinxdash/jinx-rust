#![feature(arbitrary_enum_discriminant)]                                                                                                  /*
#![feature(arbitrary_enum_discriminant)]↲    <Program>
#![feature(arbitrary_enum_discriminant)]     Attribute{inner}
  [feature(arbitrary_enum_discriminant)]     Attribute.segments{dk: "[]"}
          (arbitrary_enum_discriminant)      DelimGroup                                                                                   */

enum Enum { Unit = 1, Tuple() = 2, Struct{} = 3, }                                                                                        /*
enum•Enum•{•Unit•=•1,•Tuple()•=•2,•Struct{}•=•3,•}↲    <Program.ast{dk: "None"}>
enum•Enum•{•Unit•=•1,•Tuple()•=•2,•Struct{}•=•3,•}     EnumDeclaration
          {•Unit•=•1,•Tuple()•=•2,•Struct{}•=•3,•}     EnumDeclaration.members{dk: "{}"}
            Unit•=•1                                   EnumMemberDeclaration
                   1                                   Literal{kind: Integer}
                      Tuple()•=•2                      EnumMemberTupleDeclaration
                           ()                          EnumMemberTupleDeclaration.items{dk: "()"}
                                2                      Literal{kind: Integer}
                                   Struct{}•=•3        EnumMemberStructDeclaration
                                         {}            EnumMemberStructDeclaration.properties{dk: "{}"}
                                              3        Literal{kind: Integer}                                                             */
#[repr(u8)]                                                                                                                               /*
#[repr(u8)]↲    <EnumDeclaration>
#[repr(u8)]     Attribute{!inner}
 [repr(u8)]     Attribute.segments{dk: "[]"}
      (u8)      DelimGroup                                                                                                                */
enum Enum { Unit = 3, Tuple(u16) = 2, Struct { a: u8, b: u16, } = 1, }                                                                    /*
enum•Enum•{•Unit•=•3,•Tuple(u16)•=•2,•Struct•{•a:•u8,•b:•u16,•}•=•1,•}    EnumDeclaration~ownStart
          {•Unit•=•3,•Tuple(u16)•=•2,•Struct•{•a:•u8,•b:•u16,•}•=•1,•}    EnumDeclaration.members{dk: "{}"}
            Unit•=•3                                                      EnumMemberDeclaration
                   3                                                      Literal{kind: Integer}
                      Tuple(u16)•=•2                                      EnumMemberTupleDeclaration
                           (u16)                                          EnumMemberTupleDeclaration.items{dk: "()"}
                            u16                                           TupleStructItemDeclaration
                                   2                                      Literal{kind: Integer}
                                      Struct•{•a:•u8,•b:•u16,•}•=•1       EnumMemberStructDeclaration
                                             {•a:•u8,•b:•u16,•}           EnumMemberStructDeclaration.properties{dk: "{}"}
                                               a:•u8                      StructPropertyDeclaration
                                                      b:•u16              StructPropertyDeclaration
                                                                  1       Literal{kind: Integer}
enum•Enum•{•Unit•=•3,•Tuple(u16)•=•2,•Struct•{•a:•u8,•b:•u16,•}•=•1,•}    </EnumDeclaration>                                              */
#[repr(i8)]                                                                                                                               /*
#[repr(i8)]↲    <EnumDeclaration>
#[repr(i8)]     Attribute{!inner}
 [repr(i8)]     Attribute.segments{dk: "[]"}
      (i8)      DelimGroup                                                                                                                */
enum E2 { A = 7, B = -2, }                                                                                                                /*
enum•E2•{•A•=•7,•B•=•-2,•}    EnumDeclaration~ownStart
        {•A•=•7,•B•=•-2,•}    EnumDeclaration.members{dk: "{}"}
          A•=•7               EnumMemberDeclaration
              7               Literal{kind: Integer}
                 B•=•-2       EnumMemberDeclaration
                     -2       MinusExpression
                      2       Literal{kind: Integer}
enum•E2•{•A•=•7,•B•=•-2,•}    </EnumDeclaration>                                                                                          */
#[repr(C)]                                                                                                                                /*
#[repr(C)]↲    <EnumDeclaration>
#[repr(C)]     Attribute{!inner}
 [repr(C)]     Attribute.segments{dk: "[]"}
      (C)      DelimGroup                                                                                                                 */
enum E3 { A = 42, B = 100, }                                                                                                              /*
enum•E3•{•A•=•42,•B•=•100,•}    EnumDeclaration~ownStart
        {•A•=•42,•B•=•100,•}    EnumDeclaration.members{dk: "{}"}
          A•=•42                EnumMemberDeclaration
              42                Literal{kind: Integer}
                  B•=•100       EnumMemberDeclaration
                      100       Literal{kind: Integer}
enum•E3•{•A•=•42,•B•=•100,•}    </EnumDeclaration>                                                                                        */
#[repr(i128)]                                                                                                                             /*
#[repr(i128)]↲    <EnumDeclaration>
#[repr(i128)]     Attribute{!inner}
 [repr(i128)]     Attribute.segments{dk: "[]"}
      (i128)      DelimGroup                                                                                                              */
enum E4 { A = 0x1223_3445_5667_7889, B = -0x1223_3445_5667_7889, }                                                                        /*
enum•E4•{•A•=•0x1223_3445_5667_7889,•B•=•-0x1223_3445_5667_7889,•}    EnumDeclaration~ownStart
        {•A•=•0x1223_3445_5667_7889,•B•=•-0x1223_3445_5667_7889,•}    EnumDeclaration.members{dk: "{}"}
          A•=•0x1223_3445_5667_7889                                   EnumMemberDeclaration
              0x1223_3445_5667_7889                                   Literal{kind: Hex}
                                     B•=•-0x1223_3445_5667_7889       EnumMemberDeclaration
                                         -0x1223_3445_5667_7889       MinusExpression
                                          0x1223_3445_5667_7889       Literal{kind: Hex}
enum•E4•{•A•=•0x1223_3445_5667_7889,•B•=•-0x1223_3445_5667_7889,•}    </EnumDeclaration>                                                  */
enum ADT { First(u32, u32), Second(u64) }                                                                                                 /*
enum•ADT•{•First(u32,•u32),•Second(u64)•}    EnumDeclaration
         {•First(u32,•u32),•Second(u64)•}    EnumDeclaration.members{dk: "{}"}
           First(u32,•u32)                   EnumMemberTupleDeclaration
                (u32,•u32)                   EnumMemberTupleDeclaration.items{dk: "()"}
                 u32                         TupleStructItemDeclaration
                      u32                    TupleStructItemDeclaration
                            Second(u64)      EnumMemberTupleDeclaration
                                  (u64)      EnumMemberTupleDeclaration.items{dk: "()"}
                                   u64       TupleStructItemDeclaration                                                                   */
enum CLike1 { A, B, C, D }                                                                                                                /*
enum•CLike1•{•A,•B,•C,•D•}    EnumDeclaration
            {•A,•B,•C,•D•}    EnumDeclaration.members{dk: "{}"}
              A               EnumMemberDeclaration
                 B            EnumMemberDeclaration
                    C         EnumMemberDeclaration
                       D      EnumMemberDeclaration                                                                                       */
enum CLike2 { A = 5, B = 2, C = 19, D }                                                                                                   /*
enum•CLike2•{•A•=•5,•B•=•2,•C•=•19,•D•}    EnumDeclaration
            {•A•=•5,•B•=•2,•C•=•19,•D•}    EnumDeclaration.members{dk: "{}"}
              A•=•5                        EnumMemberDeclaration
                  5                        Literal{kind: Integer}
                     B•=•2                 EnumMemberDeclaration
                         2                 Literal{kind: Integer}
                            C•=•19         EnumMemberDeclaration
                                19         Literal{kind: Integer}
                                    D      EnumMemberDeclaration                                                                          */
#[repr(i8)]                                                                                                                               /*
#[repr(i8)]↲    <EnumDeclaration>
#[repr(i8)]     Attribute{!inner}
 [repr(i8)]     Attribute.segments{dk: "[]"}
      (i8)      DelimGroup                                                                                                                */
enum CLike3 { A = 5, B, C = -1, D }                                                                                                       /*
enum•CLike3•{•A•=•5,•B,•C•=•-1,•D•}    EnumDeclaration~ownStart
            {•A•=•5,•B,•C•=•-1,•D•}    EnumDeclaration.members{dk: "{}"}
              A•=•5                    EnumMemberDeclaration
                  5                    Literal{kind: Integer}
                     B                 EnumMemberDeclaration
                        C•=•-1         EnumMemberDeclaration
                            -1         MinusExpression
                             1         Literal{kind: Integer}
                                D      EnumMemberDeclaration
enum•CLike3•{•A•=•5,•B,•C•=•-1,•D•}    </EnumDeclaration>                                                                                 */
enum ADT { First(u32, u32), Second(u64) }                                                                                                 /*
enum•ADT•{•First(u32,•u32),•Second(u64)•}    EnumDeclaration
         {•First(u32,•u32),•Second(u64)•}    EnumDeclaration.members{dk: "{}"}
           First(u32,•u32)                   EnumMemberTupleDeclaration
                (u32,•u32)                   EnumMemberTupleDeclaration.items{dk: "()"}
                 u32                         TupleStructItemDeclaration
                      u32                    TupleStructItemDeclaration
                            Second(u64)      EnumMemberTupleDeclaration
                                  (u64)      EnumMemberTupleDeclaration.items{dk: "()"}
                                   u64       TupleStructItemDeclaration                                                                   */
enum NullablePointer { Something(&'static u32), Nothing }                                                                                 /*
enum•NullablePointer•{•Something(&'static•u32),•Nothing•}    EnumDeclaration
                     {•Something(&'static•u32),•Nothing•}    EnumDeclaration.members{dk: "{}"}
                       Something(&'static•u32)               EnumMemberTupleDeclaration
                                (&'static•u32)               EnumMemberTupleDeclaration.items{dk: "()"}
                                 &'static•u32                TupleStructItemDeclaration, TypeReference{!mut}
                                  'static                    LtStatic
                                                Nothing      EnumMemberDeclaration                                                        */
#[repr(isize)]                                                                                                                            /*
#[repr(isize)]↲    <EnumDeclaration>
#[repr(isize)]     Attribute{!inner}
 [repr(isize)]     Attribute.segments{dk: "[]"}
      (isize)      DelimGroup                                                                                                             */
enum Mixed { Unit = 3, Tuple(u16) = 2, Struct { a: u8, b: u16, } = 1, }                                                                   /*
enum•Mixed•{•Unit•=•3,•Tuple(u16)•=•2,•Struct•{•a:•u8,•b:•u16,•}•=•1,•}    EnumDeclaration~ownStart
           {•Unit•=•3,•Tuple(u16)•=•2,•Struct•{•a:•u8,•b:•u16,•}•=•1,•}    EnumDeclaration.members{dk: "{}"}
             Unit•=•3                                                      EnumMemberDeclaration
                    3                                                      Literal{kind: Integer}
                       Tuple(u16)•=•2                                      EnumMemberTupleDeclaration
                            (u16)                                          EnumMemberTupleDeclaration.items{dk: "()"}
                             u16                                           TupleStructItemDeclaration
                                    2                                      Literal{kind: Integer}
                                       Struct•{•a:•u8,•b:•u16,•}•=•1       EnumMemberStructDeclaration
                                              {•a:•u8,•b:•u16,•}           EnumMemberStructDeclaration.properties{dk: "{}"}
                                                a:•u8                      StructPropertyDeclaration
                                                       b:•u16              StructPropertyDeclaration
                                                                   1       Literal{kind: Integer}
enum•Mixed•{•Unit•=•3,•Tuple(u16)•=•2,•Struct•{•a:•u8,•b:•u16,•}•=•1,•}    </EnumDeclaration>                                             */
enum MyWeirdOption<T> { None = 0, Some(T) = std::mem::size_of::<T>(), }                                                                   /*
enum•MyWeirdOption<T>•{•None•=•0,•Some(T)•=•std::mem::size_of::<T>(),•}    EnumDeclaration
                  <T>                                                      EnumDeclaration.generics{dk: "<>"}
                   T                                                       GenericTypeParameterDeclaration
                      {•None•=•0,•Some(T)•=•std::mem::size_of::<T>(),•}    EnumDeclaration.members{dk: "{}"}
                        None•=•0                                           EnumMemberDeclaration
                               0                                           Literal{kind: Integer}
                                  Some(T)•=•std::mem::size_of::<T>()       EnumMemberTupleDeclaration
                                      (T)                                  EnumMemberTupleDeclaration.items{dk: "()"}
                                       T                                   TupleStructItemDeclaration
                                            std::mem::size_of::<T>()       CallExpression
                                            std::mem::size_of              ExpressionPath
                                            std::mem                       ExpressionPath
                                                               <T>         CallExpression.typeArguments{dk: "<>"}
                                                                  ()       CallExpression.arguments{dk: "()"}                             */
enum Test { A(Box<u64>) = 0, B(usize) = u64::MAX as i128 + 1, }                                                                           /*
enum•Test•{•A(Box<u64>)•=•0,•B(usize)•=•u64::MAX•as•i128•+•1,•}    EnumDeclaration
          {•A(Box<u64>)•=•0,•B(usize)•=•u64::MAX•as•i128•+•1,•}    EnumDeclaration.members{dk: "{}"}
            A(Box<u64>)•=•0                                        EnumMemberTupleDeclaration
             (Box<u64>)                                            EnumMemberTupleDeclaration.items{dk: "()"}
              Box<u64>                                             TupleStructItemDeclaration, TypeCall
                 <u64>                                             TypeCall.typeArguments{dk: "<>"}
                          0                                        Literal{kind: Integer}
                             B(usize)•=•u64::MAX•as•i128•+•1       EnumMemberTupleDeclaration
                              (usize)                              EnumMemberTupleDeclaration.items{dk: "()"}
                               usize                               TupleStructItemDeclaration
                                        u64::MAX•as•i128•+•1       OperationExpression{tk: "+"}
                                        u64::MAX•as•i128           ExpressionAsTypeCast
                                        u64::MAX                   ExpressionPath
                                                           1       Literal{kind: Integer}                                                 */
pub enum Foo { A = 2, }                                                                                                                   /*
pub•enum•Foo•{•A•=•2,•}    EnumDeclaration
pub                        PubSpecifier
             {•A•=•2,•}    EnumDeclaration.members{dk: "{}"}
               A•=•2       EnumMemberDeclaration
                   2       Literal{kind: Integer}                                                                                         */
pub enum Bar { A(Foo), B, C, }                                                                                                            /*
pub•enum•Bar•{•A(Foo),•B,•C,•}    EnumDeclaration
pub                               PubSpecifier
             {•A(Foo),•B,•C,•}    EnumDeclaration.members{dk: "{}"}
               A(Foo)             EnumMemberTupleDeclaration
                (Foo)             EnumMemberTupleDeclaration.items{dk: "()"}
                 Foo              TupleStructItemDeclaration
                       B          EnumMemberDeclaration
                          C       EnumMemberDeclaration                                                                                   */
pub enum Size { One = 1, Two = 2, Three = 3, }                                                                                            /*
pub•enum•Size•{•One•=•1,•Two•=•2,•Three•=•3,•}    EnumDeclaration
pub                                               PubSpecifier
              {•One•=•1,•Two•=•2,•Three•=•3,•}    EnumDeclaration.members{dk: "{}"}
                One•=•1                           EnumMemberDeclaration
                      1                           Literal{kind: Integer}
                         Two•=•2                  EnumMemberDeclaration
                               2                  Literal{kind: Integer}
                                  Three•=•3       EnumMemberDeclaration
                                          3       Literal{kind: Integer}                                                                  */
#[repr(i128)]                                                                                                                             /*
#[repr(i128)]↲    <EnumDeclaration>
#[repr(i128)]     Attribute{!inner}
 [repr(i128)]     Attribute.segments{dk: "[]"}
      (i128)      DelimGroup                                                                                                              */
enum Signed {                                                                                                                             /*
enum•Signed•{↲    <EnumDeclaration~ownStart>
            {↲    <EnumDeclaration.members{dk: "{}"}>                                                                                     */
    Zero = 0,                                                                                                                             /*
    Zero•=•0    EnumMemberDeclaration
           0    Literal{kind: Integer}                                                                                                    */
    Staircase = 0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f,                                                                           /*
    Staircase•=•0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f    EnumMemberDeclaration
                0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f    Literal{kind: Hex}                                                      */
    U64Limit = u64::MAX as i128 + 1,                                                                                                      /*
    U64Limit•=•u64::MAX•as•i128•+•1    EnumMemberDeclaration
               u64::MAX•as•i128•+•1    OperationExpression{tk: "+"}
               u64::MAX•as•i128        ExpressionAsTypeCast
               u64::MAX                ExpressionPath
                                  1    Literal{kind: Integer}                                                                             */
    SmallNegative = -1,                                                                                                                   /*
    SmallNegative•=•-1    EnumMemberDeclaration
                    -1    MinusExpression
                     1    Literal{kind: Integer}                                                                                          */
    BigNegative = i128::MIN,                                                                                                              /*
    BigNegative•=•i128::MIN    EnumMemberDeclaration
                  i128::MIN    ExpressionPath                                                                                             */
    Next,                                                                                                                                 /*
    Next    EnumMemberDeclaration                                                                                                         */
}                                                                                                                                         /*
}    </EnumDeclaration.members>
}    </EnumDeclaration>                                                                                                                   */
#[repr(u128)]                                                                                                                             /*
#[repr(u128)]↲    <EnumDeclaration>
#[repr(u128)]     Attribute{!inner}
 [repr(u128)]     Attribute.segments{dk: "[]"}
      (u128)      DelimGroup                                                                                                              */
enum Unsigned {                                                                                                                           /*
enum•Unsigned•{↲    <EnumDeclaration~ownStart>
              {↲    <EnumDeclaration.members{dk: "{}"}>                                                                                   */
    Zero = 0,                                                                                                                             /*
    Zero•=•0    EnumMemberDeclaration
           0    Literal{kind: Integer}                                                                                                    */
    Staircase = 0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f,                                                                           /*
    Staircase•=•0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f    EnumMemberDeclaration
                0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f    Literal{kind: Hex}                                                      */
    U64Limit = u64::MAX as u128 + 1,                                                                                                      /*
    U64Limit•=•u64::MAX•as•u128•+•1    EnumMemberDeclaration
               u64::MAX•as•u128•+•1    OperationExpression{tk: "+"}
               u64::MAX•as•u128        ExpressionAsTypeCast
               u64::MAX                ExpressionPath
                                  1    Literal{kind: Integer}                                                                             */
    Next,                                                                                                                                 /*
    Next    EnumMemberDeclaration                                                                                                         */
}                                                                                                                                         /*
}    </EnumDeclaration.members>
}    </EnumDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 0
// Parsed Nodes: 297
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 1603 (25% re-reads)
// Unnecessary 'skip_whitespace()' calls: 160
// source: "../../samples/features/arbitrary_enum_discriminant.rs"