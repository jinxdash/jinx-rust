fn f(self) {}                                                                                                                             /*
fn•f(self)•{}↲    <Program>
fn•f(self)•{}↲    <Program.ast{dk: "None"}>
fn•f(self)•{}     FunctionDeclaration
    (self)        FunctionDeclaration.parameters{dk: "()"}
     self         FunctionSelfParameterDeclaration{!ref, !mut}
           {}     FunctionDeclaration.body{dk: "{}"}                                                                                      */
fn f(&self) {}                                                                                                                            /*
fn•f(&self)•{}    FunctionDeclaration
    (&self)       FunctionDeclaration.parameters{dk: "()"}
     &self        FunctionSelfParameterDeclaration{ref, !mut}
            {}    FunctionDeclaration.body{dk: "{}"}                                                                                      */
fn f(mut self) {}                                                                                                                         /*
fn•f(mut•self)•{}    FunctionDeclaration
    (mut•self)       FunctionDeclaration.parameters{dk: "()"}
     mut•self        FunctionSelfParameterDeclaration{!ref, mut}
               {}    FunctionDeclaration.body{dk: "{}"}                                                                                   */
fn f(&mut self) {}                                                                                                                        /*
fn•f(&mut•self)•{}    FunctionDeclaration
    (&mut•self)       FunctionDeclaration.parameters{dk: "()"}
     &mut•self        FunctionSelfParameterDeclaration{ref, mut}
                {}    FunctionDeclaration.body{dk: "{}"}                                                                                  */
fn f(&'a self) {}                                                                                                                         /*
fn•f(&'a•self)•{}    FunctionDeclaration
    (&'a•self)       FunctionDeclaration.parameters{dk: "()"}
     &'a•self        FunctionSelfParameterDeclaration{ref, !mut}
      'a             LtIdentifier
               {}    FunctionDeclaration.body{dk: "{}"}                                                                                   */
fn f(&'a mut self) {}                                                                                                                     /*
fn•f(&'a•mut•self)•{}    FunctionDeclaration
    (&'a•mut•self)       FunctionDeclaration.parameters{dk: "()"}
     &'a•mut•self        FunctionSelfParameterDeclaration{ref, mut}
      'a                 LtIdentifier
                   {}    FunctionDeclaration.body{dk: "{}"}                                                                               */
fn f(self: u8) {}                                                                                                                         /*
fn•f(self:•u8)•{}    FunctionDeclaration
    (self:•u8)       FunctionDeclaration.parameters{dk: "()"}
     self:•u8        FunctionSelfParameterDeclaration{!ref, !mut}
               {}    FunctionDeclaration.body{dk: "{}"}                                                                                   */
fn f(mut self: u8) {}                                                                                                                     /*
fn•f(mut•self:•u8)•{}    FunctionDeclaration
    (mut•self:•u8)       FunctionDeclaration.parameters{dk: "()"}
     mut•self:•u8        FunctionSelfParameterDeclaration{!ref, mut}
                   {}    FunctionDeclaration.body{dk: "{}"}                                                                               */
type X = fn(self);                                                                                                                        /*
type•X•=•fn(self);    TypeAliasDeclaration
         fn(self)     TypeFnPointer
           (self)     TypeFnPointer.parameters{dk: "()"}
            self      TypeFnPointerParameter                                                                                              */
type X = fn(&self);                                                                                                                       /*
type•X•=•fn(&self);    TypeAliasDeclaration
         fn(&self)     TypeFnPointer
           (&self)     TypeFnPointer.parameters{dk: "()"}
            &self      TypeFnPointerParameter, TypeReference{!mut}                                                                        */
// type X = fn(mut self);
//•type•X•=•fn(mut•self);    Comment{line}
type X = fn(&mut self);                                                                                                                   /*
type•X•=•fn(&mut•self);    TypeAliasDeclaration
         fn(&mut•self)     TypeFnPointer
           (&mut•self)     TypeFnPointer.parameters{dk: "()"}
            &mut•self      TypeFnPointerParameter, TypeReference{mut}                                                                     */
type X = fn(&'a self);                                                                                                                    /*
type•X•=•fn(&'a•self);    TypeAliasDeclaration
         fn(&'a•self)     TypeFnPointer
           (&'a•self)     TypeFnPointer.parameters{dk: "()"}
            &'a•self      TypeFnPointerParameter, TypeReference{!mut}
             'a           LtIdentifier                                                                                                    */
type X = fn(&'a mut self);                                                                                                                /*
type•X•=•fn(&'a•mut•self);    TypeAliasDeclaration
         fn(&'a•mut•self)     TypeFnPointer
           (&'a•mut•self)     TypeFnPointer.parameters{dk: "()"}
            &'a•mut•self      TypeFnPointerParameter, TypeReference{mut}
             'a               LtIdentifier                                                                                                */
type X = fn(self: u8);                                                                                                                    /*
type•X•=•fn(self:•u8);    TypeAliasDeclaration
         fn(self:•u8)     TypeFnPointer
           (self:•u8)     TypeFnPointer.parameters{dk: "()"}
            self:•u8      TypeFnPointerParameter                                                                                          */
// type X = fn(mut self: u8);
//•type•X•=•fn(mut•self:•u8);    Comment{line}
async fn foo<'b>(self: &'b Foo<'a>) -> &() { self.0 }                                                                                     /*
async•fn•foo<'b>(self:•&'b•Foo<'a>)•->•&()•{•self.0•}    FunctionDeclaration{async}
            <'b>                                         FunctionDeclaration.generics{dk: "<>"}
             'b                                          GenericLtParameterDeclaration, LtIdentifier
                (self:•&'b•Foo<'a>)                      FunctionDeclaration.parameters{dk: "()"}
                 self:•&'b•Foo<'a>                       FunctionSelfParameterDeclaration{!ref, !mut}
                       &'b•Foo<'a>                       TypeReference{!mut}
                        'b                               LtIdentifier
                           Foo<'a>                       TypeCall
                              <'a>                       TypeCall.typeArguments{dk: "<>"}
                               'a                        LtIdentifier
                                       &()               TypeReference{!mut}
                                        ()               TypeTuple
                                           {•self.0•}    FunctionDeclaration.body{dk: "{}"}
                                             self.0      ExpressionStatement{!semi}, MemberExpression{!computed}
                                                  0      Index                                                                            */
fn f<'b>(self: &'b Foo<'a>) -> &() { self.0 };                                                                                            /*
fn•f<'b>(self:•&'b•Foo<'a>)•->•&()•{•self.0•}     FunctionDeclaration
    <'b>                                          FunctionDeclaration.generics{dk: "<>"}
     'b                                           GenericLtParameterDeclaration, LtIdentifier
        (self:•&'b•Foo<'a>)                       FunctionDeclaration.parameters{dk: "()"}
         self:•&'b•Foo<'a>                        FunctionSelfParameterDeclaration{!ref, !mut}
               &'b•Foo<'a>                        TypeReference{!mut}
                'b                                LtIdentifier
                   Foo<'a>                        TypeCall
                      <'a>                        TypeCall.typeArguments{dk: "<>"}
                       'a                         LtIdentifier
                               &()                TypeReference{!mut}
                                ()                TypeTuple
                                   {•self.0•}     FunctionDeclaration.body{dk: "{}"}
                                     self.0       ExpressionStatement{!semi}, MemberExpression{!computed}
                                          0       Index
                                             ;    ExpressionStatement{semi}                                                               */
fn f<'a>(self: &Alias, arg: &'a ()) -> &() { arg }                                                                                        /*
fn•f<'a>(self:•&Alias,•arg:•&'a•())•->•&()•{•arg•}    FunctionDeclaration
    <'a>                                              FunctionDeclaration.generics{dk: "<>"}
     'a                                               GenericLtParameterDeclaration, LtIdentifier
        (self:•&Alias,•arg:•&'a•())                   FunctionDeclaration.parameters{dk: "()"}
         self:•&Alias                                 FunctionSelfParameterDeclaration{!ref, !mut}
               &Alias                                 TypeReference{!mut}
                       arg:•&'a•()                    FunctionParameterDeclaration
                            &'a•()                    TypeReference{!mut}
                             'a                       LtIdentifier
                                ()                    TypeTuple
                                       &()            TypeReference{!mut}
                                        ()            TypeTuple
                                           {•arg•}    FunctionDeclaration.body{dk: "{}"}
                                             arg      ExpressionStatement{!semi}                                                          */
fn f(&mut self) -> u32;                                                                                                                   /*
fn•f(&mut•self)•->•u32;    FunctionDeclaration
    (&mut•self)            FunctionDeclaration.parameters{dk: "()"}
     &mut•self             FunctionSelfParameterDeclaration{ref, mut}                                                                     */
fn f(mut self: Box<Self>);                                                                                                                /*
fn•f(mut•self:•Box<Self>);    FunctionDeclaration
    (mut•self:•Box<Self>)     FunctionDeclaration.parameters{dk: "()"}
     mut•self:•Box<Self>      FunctionSelfParameterDeclaration{!ref, mut}
               Box<Self>      TypeCall
                  <Self>      TypeCall.typeArguments{dk: "<>"}                                                                            */
fn f(self: _) {}                                                                                                                          /*
fn•f(self:•_)•{}    FunctionDeclaration
    (self:•_)       FunctionDeclaration.parameters{dk: "()"}
     self:•_        FunctionSelfParameterDeclaration{!ref, !mut}
           _        TypeInferred
              {}    FunctionDeclaration.body{dk: "{}"}                                                                                    */
fn f(self: &_) {}                                                                                                                         /*
fn•f(self:•&_)•{}    FunctionDeclaration
    (self:•&_)       FunctionDeclaration.parameters{dk: "()"}
     self:•&_        FunctionSelfParameterDeclaration{!ref, !mut}
           &_        TypeReference{!mut}
            _        TypeInferred
               {}    FunctionDeclaration.body{dk: "{}"}                                                                                   */
fn f(&self) -> Self;                                                                                                                      /*
fn•f(&self)•->•Self;    FunctionDeclaration
    (&self)             FunctionDeclaration.parameters{dk: "()"}
     &self              FunctionSelfParameterDeclaration{ref, !mut}
fn•f(&self)•->•Self;    </Program.ast>                                                                                                    */
// fn f(self::S: S) {}
//•fn•f(self::S:•S)•{}    Comment{line}
// fn g(&self::S: &S) {}
//•fn•g(&self::S:•&S)•{}    Comment{line}
// fn h(&mut self::S: &mut S) {}
//•fn•h(&mut•self::S:•&mut•S)•{}    Comment{line}
//•fn•h(&mut•self::S:•&mut•S)•{}    </Program>
// Discarded Nodes: 0
// Parsed Nodes: 144
// state_rollbacks: 3
// Total '.charCodeAt()' calls: 928 (37% re-reads)
// Unnecessary 'skip_whitespace()' calls: 90
// source: "../../samples/statements/self.rs"