# shunting-yard-parser
A basic shunting yard parser that parses simplistic arithmetic expressions by converting infix expressions to reverse polish notation and evaluating it.

# Preface:
This is a basic parser written in rust that uses a basic ass-pulled hand-written lexer with the usage of the shunting yard algorithm.

# What is the shunting yard parser?
The shunting yard parser is an algorithm made in 1961 made by Edgar W. Dijkstra introduced in the mathematisch centrum paper. It was introduced as a less resource intensive algorithm for parsing infix expressions by converting them from infix notation to reverse polish notation (or an abstract syntax tree). This algorithm was later generalized into operator-precedence parsing with LR(k) ( LR(1) ) by using basic shift-reduction parsing with the usage of a stack, which is what this algorithm uses.

# Infix notation & reverse polish notation:
In the previous paragraph I mentioned infix notation, so what exactly is that? What does it look like? Well, infix notation is the most popular form of writing arithmetic expressions and looks like such: ``1 + 1``, ``10 * 2 + 5``, ``129 * 2``, et cetera you get the idea. Now that we've went over that, what's reverse polish notation, what does it look like & why do we care? Reverse polish notation is another notation for writing expressions and it looks like such: ``1 1 +``, ``10 2 * 5 +``, ``129 2 *``. The reason we care is because this notation is naturally easy for computers to evaluate these expressions, they naturally take the form of an abstract syntax tree. We can convert any given reverse polish notation into an abstract syntax tree as it grows on them.
