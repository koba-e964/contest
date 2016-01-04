/**
 * Enumerates all factors of the given integer(Long).
 * Verified by: AtCoder ARC026-B (http://arc026.contest.atcoder.jp/submissions/603926)
 */
final object AllFactors {
  def apply(a: Long): Array[Long] = {
    val buf = scala.collection.mutable.ArrayBuffer[Long]()
    var p: Long = 1
    while (p * p < a) {
      if (a % p == 0) {
        buf += p
      }
      p += 1
    }
    if (p * p == a) {
      buf ++= Array(p) ++ buf.toArray.reverse.map(a / _)
    } else {
      buf ++= buf.toArray.reverse.map(a / _)
    }
    buf.toArray
  }
}
