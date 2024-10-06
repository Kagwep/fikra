$$

\begin{align}

[\text{Prog}] &\to [\text{Stmt}]^* \\

[\text{Stmt}^*] &\to

\begin{cases}

\text{return} [\text{Expr}]; \\ 
\text{let}\space\text{ident} = [\text{Expr}]

\end{cases}

\\

[\text{Expr}] &\to 

\begin{cases}

 [\text{Term}] \\
 [\text{BinExpr}]
\end{cases} \\

[\text{BinExpr}] &\to

\begin{cases}

[\text{Expr}] * [\text{Expr}] & \text{prec} = 2 \\

[\text{Expr}] + [\text{Expr}] &  \text{prec} = 3

\end{cases} \\

[\text{Term}] &\to

\begin{cases}
\text{int\_lit} \\
\text{ident}
\end{cases}

\end{align}

$$