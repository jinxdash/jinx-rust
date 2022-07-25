#![feature(arbitrary_enum_discriminant)]                                                                                                  /*
#![feature(arbitrary_enum_discriminant)]    Attribute
          (arbitrary_enum_discriminant)     DelimGroup                                                                                    */

enum Enum { Unit = 1, Tuple() = 2, Struct{} = 3, }                                                                                        /*
enum•Enum•{•Unit•=•1,•Tuple()•=•2,•Struct{}•=•3,•}    EnumDeclaration
            Unit•=•1                                  EnumMemberDeclaration
                   1                                  Literal
                      Tuple()•=•2                     EnumMemberTupleDeclaration
                                2                     Literal
                                   Struct{}•=•3       EnumMemberStructDeclaration
                                              3       Literal                                                                             */
#[repr(u8)]                                                                                                                               /*
#[repr(u8)]↲    <EnumDeclaration>
#[repr(u8)]     Attribute
      (u8)      DelimGroup                                                                                                                */
enum Enum { Unit = 3, Tuple(u16) = 2, Struct { a: u8, b: u16, } = 1, }                                                                    /*
enum•Enum•{•Unit•=•3,•Tuple(u16)•=•2,•Struct•{•a:•u8,•b:•u16,•}•=•1,•}    </EnumDeclaration>
            Unit•=•3                                                      EnumMemberDeclaration
                   3                                                      Literal
                      Tuple(u16)•=•2                                      EnumMemberTupleDeclaration
                            u16                                           TupleStructItemDeclaration
                                   2                                      Literal
                                      Struct•{•a:•u8,•b:•u16,•}•=•1       EnumMemberStructDeclaration
                                               a:•u8                      StructPropertyDeclaration
                                                      b:•u16              StructPropertyDeclaration
                                                                  1       Literal                                                         */
#[repr(i8)]                                                                                                                               /*
#[repr(i8)]↲    <EnumDeclaration>
#[repr(i8)]     Attribute
      (i8)      DelimGroup                                                                                                                */
enum E2 { A = 7, B = -2, }                                                                                                                /*
enum•E2•{•A•=•7,•B•=•-2,•}    </EnumDeclaration>
          A•=•7               EnumMemberDeclaration
              7               Literal
                 B•=•-2       EnumMemberDeclaration
                     -2       MinusExpression
                      2       Literal                                                                                                     */
#[repr(C)]                                                                                                                                /*
#[repr(C)]↲    <EnumDeclaration>
#[repr(C)]     Attribute
      (C)      DelimGroup                                                                                                                 */
enum E3 { A = 42, B = 100, }                                                                                                              /*
enum•E3•{•A•=•42,•B•=•100,•}    </EnumDeclaration>
          A•=•42                EnumMemberDeclaration
              42                Literal
                  B•=•100       EnumMemberDeclaration
                      100       Literal                                                                                                   */
#[repr(i128)]                                                                                                                             /*
#[repr(i128)]↲    <EnumDeclaration>
#[repr(i128)]     Attribute
      (i128)      DelimGroup                                                                                                              */
enum E4 { A = 0x1223_3445_5667_7889, B = -0x1223_3445_5667_7889, }                                                                        /*
enum•E4•{•A•=•0x1223_3445_5667_7889,•B•=•-0x1223_3445_5667_7889,•}    </EnumDeclaration>
          A•=•0x1223_3445_5667_7889                                   EnumMemberDeclaration
              0x1223_3445_5667_7889                                   Literal
                                     B•=•-0x1223_3445_5667_7889       EnumMemberDeclaration
                                         -0x1223_3445_5667_7889       MinusExpression
                                          0x1223_3445_5667_7889       Literal                                                             */
enum ADT { First(u32, u32), Second(u64) }                                                                                                 /*
enum•ADT•{•First(u32,•u32),•Second(u64)•}    EnumDeclaration
           First(u32,•u32)                   EnumMemberTupleDeclaration
                 u32                         TupleStructItemDeclaration
                      u32                    TupleStructItemDeclaration
                            Second(u64)      EnumMemberTupleDeclaration
                                   u64       TupleStructItemDeclaration                                                                   */
enum CLike1 { A, B, C, D }                                                                                                                /*
enum•CLike1•{•A,•B,•C,•D•}    EnumDeclaration
              A               EnumMemberDeclaration
                 B            EnumMemberDeclaration
                    C         EnumMemberDeclaration
                       D      EnumMemberDeclaration                                                                                       */
enum CLike2 { A = 5, B = 2, C = 19, D }                                                                                                   /*
enum•CLike2•{•A•=•5,•B•=•2,•C•=•19,•D•}    EnumDeclaration
              A•=•5                        EnumMemberDeclaration
                  5                        Literal
                     B•=•2                 EnumMemberDeclaration
                         2                 Literal
                            C•=•19         EnumMemberDeclaration
                                19         Literal
                                    D      EnumMemberDeclaration                                                                          */
#[repr(i8)]                                                                                                                               /*
#[repr(i8)]↲    <EnumDeclaration>
#[repr(i8)]     Attribute
      (i8)      DelimGroup                                                                                                                */
enum CLike3 { A = 5, B, C = -1, D }                                                                                                       /*
enum•CLike3•{•A•=•5,•B,•C•=•-1,•D•}    </EnumDeclaration>
              A•=•5                    EnumMemberDeclaration
                  5                    Literal
                     B                 EnumMemberDeclaration
                        C•=•-1         EnumMemberDeclaration
                            -1         MinusExpression
                             1         Literal
                                D      EnumMemberDeclaration                                                                              */
enum ADT { First(u32, u32), Second(u64) }                                                                                                 /*
enum•ADT•{•First(u32,•u32),•Second(u64)•}    EnumDeclaration
           First(u32,•u32)                   EnumMemberTupleDeclaration
                 u32                         TupleStructItemDeclaration
                      u32                    TupleStructItemDeclaration
                            Second(u64)      EnumMemberTupleDeclaration
                                   u64       TupleStructItemDeclaration                                                                   */
enum NullablePointer { Something(&'static u32), Nothing }                                                                                 /*
enum•NullablePointer•{•Something(&'static•u32),•Nothing•}    EnumDeclaration
                       Something(&'static•u32)               EnumMemberTupleDeclaration
                                 &'static•u32                TupleStructItemDeclaration, TypeReference
                                  'static                    LtStatic
                                                Nothing      EnumMemberDeclaration                                                        */
#[repr(isize)]                                                                                                                            /*
#[repr(isize)]↲    <EnumDeclaration>
#[repr(isize)]     Attribute
      (isize)      DelimGroup                                                                                                             */
enum Mixed { Unit = 3, Tuple(u16) = 2, Struct { a: u8, b: u16, } = 1, }                                                                   /*
enum•Mixed•{•Unit•=•3,•Tuple(u16)•=•2,•Struct•{•a:•u8,•b:•u16,•}•=•1,•}    </EnumDeclaration>
             Unit•=•3                                                      EnumMemberDeclaration
                    3                                                      Literal
                       Tuple(u16)•=•2                                      EnumMemberTupleDeclaration
                             u16                                           TupleStructItemDeclaration
                                    2                                      Literal
                                       Struct•{•a:•u8,•b:•u16,•}•=•1       EnumMemberStructDeclaration
                                                a:•u8                      StructPropertyDeclaration
                                                       b:•u16              StructPropertyDeclaration
                                                                   1       Literal                                                        */
enum MyWeirdOption<T> { None = 0, Some(T) = std::mem::size_of::<T>(), }                                                                   /*
enum•MyWeirdOption<T>•{•None•=•0,•Some(T)•=•std::mem::size_of::<T>(),•}    EnumDeclaration
                   T                                                       GenericTypeParameterDeclaration
                        None•=•0                                           EnumMemberDeclaration
                               0                                           Literal
                                  Some(T)•=•std::mem::size_of::<T>()       EnumMemberTupleDeclaration
                                       T                                   TupleStructItemDeclaration
                                            std::mem::size_of::<T>()       CallExpression
                                            std::mem::size_of              ExpressionPath
                                            std::mem                       ExpressionPath                                                 */
enum Test { A(Box<u64>) = 0, B(usize) = u64::MAX as i128 + 1, }                                                                           /*
enum•Test•{•A(Box<u64>)•=•0,•B(usize)•=•u64::MAX•as•i128•+•1,•}    EnumDeclaration
            A(Box<u64>)•=•0                                        EnumMemberTupleDeclaration
              Box<u64>                                             TupleStructItemDeclaration, TypeCall
                          0                                        Literal
                             B(usize)•=•u64::MAX•as•i128•+•1       EnumMemberTupleDeclaration
                               usize                               TupleStructItemDeclaration
                                        u64::MAX•as•i128•+•1       OperationExpression
                                        u64::MAX•as•i128           ExpressionAsTypeCast
                                        u64::MAX                   ExpressionPath
                                                           1       Literal                                                                */
pub enum Foo { A = 2, }                                                                                                                   /*
pub•enum•Foo•{•A•=•2,•}    EnumDeclaration
pub                        PubSpecifier
               A•=•2       EnumMemberDeclaration
                   2       Literal                                                                                                        */
pub enum Bar { A(Foo), B, C, }                                                                                                            /*
pub•enum•Bar•{•A(Foo),•B,•C,•}    EnumDeclaration
pub                               PubSpecifier
               A(Foo)             EnumMemberTupleDeclaration
                 Foo              TupleStructItemDeclaration
                       B          EnumMemberDeclaration
                          C       EnumMemberDeclaration                                                                                   */
pub enum Size { One = 1, Two = 2, Three = 3, }                                                                                            /*
pub•enum•Size•{•One•=•1,•Two•=•2,•Three•=•3,•}    EnumDeclaration
pub                                               PubSpecifier
                One•=•1                           EnumMemberDeclaration
                      1                           Literal
                         Two•=•2                  EnumMemberDeclaration
                               2                  Literal
                                  Three•=•3       EnumMemberDeclaration
                                          3       Literal                                                                                 */
#[repr(i128)]                                                                                                                             /*
#[repr(i128)]↲    <EnumDeclaration>
#[repr(i128)]     Attribute
      (i128)      DelimGroup                                                                                                              */
enum Signed {
    Zero = 0,                                                                                                                             /*
    Zero•=•0     EnumMemberDeclaration
           0     Literal                                                                                                                  */
    Staircase = 0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f,                                                                           /*
    Staircase•=•0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f     EnumMemberDeclaration
                0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f     Literal                                                                */
    U64Limit = u64::MAX as i128 + 1,                                                                                                      /*
    U64Limit•=•u64::MAX•as•i128•+•1     EnumMemberDeclaration
               u64::MAX•as•i128•+•1     OperationExpression
               u64::MAX•as•i128         ExpressionAsTypeCast
               u64::MAX                 ExpressionPath
                                  1     Literal                                                                                           */
    SmallNegative = -1,                                                                                                                   /*
    SmallNegative•=•-1     EnumMemberDeclaration
                    -1     MinusExpression
                     1     Literal                                                                                                        */
    BigNegative = i128::MIN,                                                                                                              /*
    BigNegative•=•i128::MIN     EnumMemberDeclaration
                  i128::MIN     ExpressionPath                                                                                            */
    Next,                                                                                                                                 /*
    Next     EnumMemberDeclaration                                                                                                        */
}                                                                                                                                         /*
}    </EnumDeclaration>                                                                                                                   */
#[repr(u128)]                                                                                                                             /*
#[repr(u128)]↲    <EnumDeclaration>
#[repr(u128)]     Attribute
      (u128)      DelimGroup                                                                                                              */
enum Unsigned {
    Zero = 0,                                                                                                                             /*
    Zero•=•0     EnumMemberDeclaration
           0     Literal                                                                                                                  */
    Staircase = 0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f,                                                                           /*
    Staircase•=•0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f     EnumMemberDeclaration
                0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f     Literal                                                                */
    U64Limit = u64::MAX as u128 + 1,                                                                                                      /*
    U64Limit•=•u64::MAX•as•u128•+•1     EnumMemberDeclaration
               u64::MAX•as•u128•+•1     OperationExpression
               u64::MAX•as•u128         ExpressionAsTypeCast
               u64::MAX                 ExpressionPath
                                  1     Literal                                                                                           */
    Next,                                                                                                                                 /*
    Next     EnumMemberDeclaration                                                                                                        */
}                                                                                                                                         /*
}    </EnumDeclaration>                                                                                                                   */

// Discarded Nodes: 0
// Parsed Nodes: 297
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 1603 (25% re-reads)
// Unnecessary 'skip_whitespace()' calls: 160
// source: "../../samples/features/arbitrary_enum_discriminant.rs"