fn f(self) {}                                                                                                                             /*
fn•f(self)•{}    FunctionDeclaration
     self        FunctionSelfParameterDeclaration                                                                                         */
fn f(&self) {}                                                                                                                            /*
fn•f(&self)•{}    FunctionDeclaration
     &self        FunctionSelfParameterDeclaration                                                                                        */
fn f(mut self) {}                                                                                                                         /*
fn•f(mut•self)•{}    FunctionDeclaration
     mut•self        FunctionSelfParameterDeclaration                                                                                     */
fn f(&mut self) {}                                                                                                                        /*
fn•f(&mut•self)•{}    FunctionDeclaration
     &mut•self        FunctionSelfParameterDeclaration                                                                                    */
fn f(&'a self) {}                                                                                                                         /*
fn•f(&'a•self)•{}    FunctionDeclaration
     &'a•self        FunctionSelfParameterDeclaration
      'a             LtIdentifier                                                                                                         */
fn f(&'a mut self) {}                                                                                                                     /*
fn•f(&'a•mut•self)•{}    FunctionDeclaration
     &'a•mut•self        FunctionSelfParameterDeclaration
      'a                 LtIdentifier                                                                                                     */
fn f(self: u8) {}                                                                                                                         /*
fn•f(self:•u8)•{}    FunctionDeclaration
     self:•u8        FunctionSelfParameterDeclaration                                                                                     */
fn f(mut self: u8) {}                                                                                                                     /*
fn•f(mut•self:•u8)•{}    FunctionDeclaration
     mut•self:•u8        FunctionSelfParameterDeclaration                                                                                 */
type X = fn(self);                                                                                                                        /*
type•X•=•fn(self);    TypeAliasDeclaration
         fn(self)     TypeFnPointer
            self      TypeFnPointerParameter                                                                                              */
type X = fn(&self);                                                                                                                       /*
type•X•=•fn(&self);    TypeAliasDeclaration
         fn(&self)     TypeFnPointer
            &self      TypeFnPointerParameter, TypeReference                                                                              */
// type X = fn(mut self);
//•type•X•=•fn(mut•self);    Comment
type X = fn(&mut self);                                                                                                                   /*
type•X•=•fn(&mut•self);    TypeAliasDeclaration
         fn(&mut•self)     TypeFnPointer
            &mut•self      TypeFnPointerParameter, TypeReference                                                                          */
type X = fn(&'a self);                                                                                                                    /*
type•X•=•fn(&'a•self);    TypeAliasDeclaration
         fn(&'a•self)     TypeFnPointer
            &'a•self      TypeFnPointerParameter, TypeReference
             'a           LtIdentifier                                                                                                    */
type X = fn(&'a mut self);                                                                                                                /*
type•X•=•fn(&'a•mut•self);    TypeAliasDeclaration
         fn(&'a•mut•self)     TypeFnPointer
            &'a•mut•self      TypeFnPointerParameter, TypeReference
             'a               LtIdentifier                                                                                                */
type X = fn(self: u8);                                                                                                                    /*
type•X•=•fn(self:•u8);    TypeAliasDeclaration
         fn(self:•u8)     TypeFnPointer
            self:•u8      TypeFnPointerParameter                                                                                          */
// type X = fn(mut self: u8);
//•type•X•=•fn(mut•self:•u8);    Comment
async fn foo<'b>(self: &'b Foo<'a>) -> &() { self.0 }                                                                                     /*
async•fn•foo<'b>(self:•&'b•Foo<'a>)•->•&()•{•self.0•}    FunctionDeclaration
             'b                                          GenericLtParameterDeclaration, LtIdentifier
                 self:•&'b•Foo<'a>                       FunctionSelfParameterDeclaration
                       &'b•Foo<'a>                       TypeReference
                        'b                               LtIdentifier
                           Foo<'a>                       TypeCall
                               'a                        LtIdentifier
                                       &()               TypeReference
                                        ()               TypeTuple
                                             self.0      ExpressionStatement, MemberExpression
                                                  0      Index                                                                            */
fn f<'b>(self: &'b Foo<'a>) -> &() { self.0 };                                                                                            /*
fn•f<'b>(self:•&'b•Foo<'a>)•->•&()•{•self.0•}     FunctionDeclaration
     'b                                           GenericLtParameterDeclaration, LtIdentifier
         self:•&'b•Foo<'a>                        FunctionSelfParameterDeclaration
               &'b•Foo<'a>                        TypeReference
                'b                                LtIdentifier
                   Foo<'a>                        TypeCall
                       'a                         LtIdentifier
                               &()                TypeReference
                                ()                TypeTuple
                                     self.0       ExpressionStatement, MemberExpression
                                          0       Index
                                             ;    ExpressionStatement                                                                     */
fn f<'a>(self: &Alias, arg: &'a ()) -> &() { arg }                                                                                        /*
fn•f<'a>(self:•&Alias,•arg:•&'a•())•->•&()•{•arg•}    FunctionDeclaration
     'a                                               GenericLtParameterDeclaration, LtIdentifier
         self:•&Alias                                 FunctionSelfParameterDeclaration
               &Alias                                 TypeReference
                       arg:•&'a•()                    FunctionParameterDeclaration
                            &'a•()                    TypeReference
                             'a                       LtIdentifier
                                ()                    TypeTuple
                                       &()            TypeReference
                                        ()            TypeTuple
                                             arg      ExpressionStatement                                                                 */
fn f(&mut self) -> u32;                                                                                                                   /*
fn•f(&mut•self)•->•u32;    FunctionDeclaration
     &mut•self             FunctionSelfParameterDeclaration                                                                               */
fn f(mut self: Box<Self>);                                                                                                                /*
fn•f(mut•self:•Box<Self>);    FunctionDeclaration
     mut•self:•Box<Self>      FunctionSelfParameterDeclaration
               Box<Self>      TypeCall                                                                                                    */
fn f(self: _) {}                                                                                                                          /*
fn•f(self:•_)•{}    FunctionDeclaration
     self:•_        FunctionSelfParameterDeclaration
           _        TypeInferred                                                                                                          */
fn f(self: &_) {}                                                                                                                         /*
fn•f(self:•&_)•{}    FunctionDeclaration
     self:•&_        FunctionSelfParameterDeclaration
           &_        TypeReference
            _        TypeInferred                                                                                                         */
fn f(&self) -> Self;                                                                                                                      /*
fn•f(&self)•->•Self;    FunctionDeclaration
     &self              FunctionSelfParameterDeclaration                                                                                  */
// fn f(self::S: S) {}
//•fn•f(self::S:•S)•{}    Comment
// fn g(&self::S: &S) {}
//•fn•g(&self::S:•&S)•{}    Comment
// fn h(&mut self::S: &mut S) {}
//•fn•h(&mut•self::S:•&mut•S)•{}    Comment

// Discarded Nodes: 0
// Parsed Nodes: 144
// state_rollbacks: 3
// Total '.charCodeAt()' calls: 928 (37% re-reads)
// Unnecessary 'skip_whitespace()' calls: 90
// source: "../../samples/statements/self.rs"