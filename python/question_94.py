import math

def integral_area(side, base):
	height_squ = side*side - (base*base)/4
	area_squ = height_squ * base * base
	area_approx = math.floor(math.sqrt(area_squ))

	return area_squ == area_approx*area_approx


def main():
	total_perimeter = 0
	side = 1

	while True:
		# Only want odd sides
		side += 1

		if (side - 1) % 1001000000 == 0:
			print side
	
		# if integral_area(side, side):
		# 	total_perimeter += 3 * side

		if 3*side - 1 > 1000000000:
			break

		if integral_area(side, side - 1):
			total_perimeter += 3 * side - 1

		if 3*side + 1 > 1000000000:
			break

		if integral_area(side, side + 1):
			total_perimeter += 3 * side + 1


	print total_perimeter

main()
