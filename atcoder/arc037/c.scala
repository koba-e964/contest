/*
 * Reference: http://lizan.asia/blog/2012/12/11/scala-competitive/
 */

object Main extends App {
  import java.{util => ju}
  import scala.annotation._
  import scala.collection._
  import scala.collection.{mutable => mu}
  import scala.collection.JavaConverters._
  import scala.math._

  val sc = new ju.Scanner(System.in)
  val n = sc.nextInt
  val k = sc.nextLong
  val a, b = Array.fill(n)(sc.nextLong).sorted
  /* binary search */
  var lo: Long = 0
  var hi: Long = 1L << 61
  @inline
  def count(v: Long): Long = { // b.count(_ <= v)
    var lo = -1
    var hi = n
    while (hi - lo > 1) {
      val mid = (lo + hi) / 2
      if (b(mid) <= v) lo = mid
      else hi = mid
    }
    lo + 1
  }
  @inline @tailrec
  def calc(v: Long, q: Int = 0, acc: Long = 0): Long = {
    if (q == n) acc
    else calc(v, q + 1, acc + count(v / a(q)))
  }
  while (hi - lo > 1) {
    val mid = (lo + hi) / 2
    if (calc(mid) >= k) hi = mid
    else lo = mid
  }
  println(hi)
}