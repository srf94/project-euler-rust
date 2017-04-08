
partitions = [1, 1, 2, 3, 5, 7, 11, 15, 22, 30]
target_loc = 10

while True:
	pents = []
	k = 1
	while True:
		pent = k*(3*k - 1)/2
		if pent > target_loc:
			break
		pents.append(pent)

		if k > 0:
			k = -k
		else:
			k = -(k-1)

	partition_sum = 0
	for (ii, pent) in enumerate(pents):
		if ii % 4 in [0, 1]:
			partition_sum += partitions[target_loc - pent]
		else:
			partition_sum -= partitions[target_loc - pent]

	partitions.append(partition_sum)

	if partition_sum % 1000000 == 0:
		print target_loc
		print partition_sum
		print len(partitions)
		break

	target_loc += 1

