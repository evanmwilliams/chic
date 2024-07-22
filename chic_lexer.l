%{
#include <iostream>
int mylineno = 0;
%}

%option noyywrap c++

string  \"[^\n"]+\"

ws      [ \t]+

alpha   [A-Za-z]
dig     [0-9]
name    ({alpha}|{dig}|\$)({alpha}|{dig}|[_.\-/$])*
num1    [-+]?{dig}+\.?([eE][-+]?{dig}+)?
num2    [-+]?{dig}*\.{dig}+([eE][-+]?{dig}+)?
number  {num1}|{num2}

%%

{number} std::cout << "number " << YYText() << '\n';

\n mylineno++;

{name} std::cout << "name " << YYText() << '\n';

{string} std::cout << "string " << YYText() << '\n';

%%

int main()
{
  FlexLexer *lexer = new yyFlexLexer;
  while (lexer->yylex() != 0)
    ;
  return 0;
}