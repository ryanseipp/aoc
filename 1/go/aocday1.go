package aocday1

func countLargerMeasurements(depths []int32) uint32 {
	var count uint32 = 0
	prev := depths[0]

	for _, depth := range depths[1:] {
		if depth > prev {
			count++
		}
		prev = depth
	}

	return count
}

func countThreeLarger(depths []int32) uint32 {
    var summed []int32

    for i := 0; i < len(depths) - 2; i++ {
        summed = append(summed, depths[i] + depths[i + 1] + depths[i + 2])
    }

    return countLargerMeasurements(summed)
}
