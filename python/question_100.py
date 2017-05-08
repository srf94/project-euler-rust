import math

sqrt_2 = math.sqrt(2)
n = 698

while True:
	if n % 100000 == 0:
		print n

	target = n*(n-1)

	b_start = math.floor(n / sqrt_2) - 10

	if 2*b_start*(b_start - 1) > target:
		print "ALERT"

	while True:
		if 2*b_start*(b_start - 1) == target:
			print "MATCH FOUND. b: %s, n: %s" % (b_start, n)
			assert 1 == 2

		if 2*b_start*(b_start - 1) > target:
			break
		b_start += 1

	n += 1


