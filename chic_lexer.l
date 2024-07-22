%{
#include <iostream>
int mylineno = 0;
void yyerror(const char *s) {
  std::cerr << "Error: " << s << " at line " << mylineno << std::endl;
}
%}

%option noyywrap c++

STRING  \"[^\n"]+\"

WS      [ \t]+

ALPHA   [A-Za-z]
DIGIT     [0-9]
IDENTIFIER    ({ALPHA})({ALPHA}|{DIGIT}|[_.\-/$])*
INTEGER    (-?[1-9][0-9]*)|(0)

%%

{INTEGER} {std::cout << "INTEGER " << YYText() << '\n';}
0{DIGIT}+ { yyerror("Invalid number with leading zeros"); }
{INTEGER}{IDENTIFIER}+ { yyerror("Invalid number with alphabets"); }

\n mylineno++;

{IDENTIFIER} std::cout << "IDENTIFIER " << YYText() << '\n';

{STRING} std::cout << "STRING " << YYText() << '\n';

"{"     { std::cout << "OPEN_CURLY" << std::endl; }
"}"     { std::cout << "CLOSE_CURLY" << std::endl; }
"["     { std::cout << "OPEN_SQUARE" << std::endl; }
"]"     { std::cout << "CLOSE_SQUARE" << std::endl; }
"("     { std::cout << "OPEN_PAREN" << std::endl; }
")"     { std::cout << "CLOSE_PAREN" << std::endl; }
"<"     { std::cout << "OPEN_ANGLE" << std::endl; }
">"     { std::cout << "CLOSE_ANGLE" << std::endl; }
";"     { std::cout<<"SEMICOLON"<<std::endl; }
","     { std::cout<<"COMMA"<<std::endl; }
":"     { std::cout<<"COLON"<<std::endl; }

%%

int main()
{
  FlexLexer *lexer = new yyFlexLexer;
  while (lexer->yylex() != 0)
    ;
  return 0;
}