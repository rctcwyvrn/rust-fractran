# rust-fractran

Would you believe me if I told you that this `fibonacci` function actually produces the right result?

```python
def fibonacci(n):
	def is_power_of_two(counter):
		bin = "{0:b}".format(counter)
		return bin[0] == "1" and all([x == "0" for x in bin[1:]])

	# http://lomont.org/posts/2017/fractran/
	fractions = [
                (17, 65),
                (133, 34),
                (17, 19),
                (23, 17),
                (2233, 69),
                (23, 29),
                (31, 23),
                (74, 341),
                (31, 37),
                (41, 31),
                (129, 287),
                (41, 43),
                (13, 41),
                (1, 13),
                (1, 3),
            ]

	counter = 78 * pow(5,n)

	while True:
		if is_power_of_two(counter):
			return int(math.log2(counter))
		for (numerator, denominator) in fractions:
			if counter % denominator == 0:
				counter //= denominator
				counter *= numerator
				break
```

You probably wouldn't, but if you run it you'll see that it actually does work. This is [fractran](https://esolangs.org/wiki/Fractran), an esoteric programming language developed by John Conway. If you remove the `is_power_of_two` part of the `while` loop, this becomes a turing complete programming language where you can compute _anything_ given the correct list of fractions.

### How???

The magic of `fractran` fades a little bit when you realize the trick. Simply put, `fractran` is a language with registers and one instruction

- The "registers" are stored in the `counter` variable. If you prime factorize the counter, the powers of the prime factors are the values of the registers. So `24` is the same as `regs[2]=3`, `regs[3]=1`.
- Each fraction is then a conditional register addition/subtraction instruction. The denominator indicates the condition that the registers much reach as well as the subtraction (`24` in the denominator means this instruction will only be run if `regs[2]>=3` and `regs[3]>=1`). The numerator represents additions to the register values

With this knowledge the fractions seem much less obtuse and you can kinda understand how this would be Turing complete. Even still, writing a program out of a list of conditional register instructions is still non-trivial (you need to encode a program counter somehow!)

### So what is this project

I wanted to mess around with a few `fractran` interpreters to see how much it could be optimized. Obviously storing the register values directly instead of having an arbitrarly large counter value increases the performance, but how much farther can we go? Can we possibly lift the program counter out of the list of fractions and slowly lift this up to a more imperative instruction set?