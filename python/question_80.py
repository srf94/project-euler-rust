import math

total_sum = 0

for number in range(1, 100):
	if math.floor(math.sqrt(number)) ** 2 == float(number):
		print number
		continue

	top = number
	p = 0
	expansion = []
	for _ in range(100):
		# Greatest x s.t. x(20p + x) <= c
		x = 1
		while x*(20*p + x) <= top:
			x += 1
		x -= 1

		expansion.append(x)

		y = (20*p + x)*x
		remainder = top - y

		p = p*10 + x
		top = 100 * remainder
		print p

	total_sum += sum(expansion)

print total_sum


