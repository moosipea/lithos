# Lithos
Because I like fancy Greek names.
I am not sure if it is actually Turing Complete.

## Example program
```
; This small example program prints the factorial of 10

(fn factorial (_ n)
  (if-else (= n 0)
	1
    (* n (factorial (- n 1)))))

(echo (factorial 10))

```
