object SumOfMultiples {
  def sum(factors: Set[Int], limit: Int): Int = {
    if (factors.isEmpty) {
      0
    } else {
      val min = factors.min
      if (min > limit) {
        0
      } else {
        var sum = 0;
        for (n <- min until limit) {
          if (factors.exists(f => (n % f) == 0)) {
            sum += n;
          }
        }
        sum
      }
    }
  }
}
