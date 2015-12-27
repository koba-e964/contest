/*
 * This class uses scala.reflect.ClassManifest, which is deprecated in Scala 2.10, because AtCoder's Scala
 * compiler is 2.9 and does not have scala.reflect.ClassTag.
 */
final class SegTree[T: scala.reflect.ClassManifest](size: Int, op : (T, T) => T, e: T) {
  /*
   * The least power of 2 that is >= s
   */
  private[this] def ceilPow2(s: Int): Int = {
    var n = 1
    while (n < s) n *= 2
    n
  }
  private[this] val n = ceilPow2(size)
  private[this] val ary = Array.fill(2 * n - 1)(e)
  @inline
  def update(idx: Int, v: T) {
    var k = idx + n - 1
    ary(k) = v
    while (k > 0) {
      k = (k - 1) / 2
      ary(k) = op(ary(2 * k + 1), ary(2 * k + 2))
    }
  }
  @inline
  def apply(idx: Int): T = {
    if (idx < 0 || idx >= n) {
      throw new RuntimeException
    }
    ary(idx + n - 1)
  }
  @inline
  def query(a: Int, b: Int): T = querySub(a, b + 1, 0, 0, n)
  private[this] def querySub(a: Int, b: Int, k: Int, l: Int, r: Int): T = {
    if (r <= a || b <= l) return e
    if (a <= l && r <= b) return ary(k)
    val vl = querySub(a, b, 2 * k + 1, l, (l + r) / 2)
    val vr = querySub(a, b, 2 * k + 2, (l + r) / 2, r)
    op(vl, vr)
  }
}
 
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
  val n, m = sc.nextInt
  val f = Array.fill(n)(sc.nextInt)
  val dp = new SegTree[Int](n + 1, (i, j) =>  ((i + j) % 1000000007L).asInstanceOf[Int], 0)
  var p: Int = 0
  val s = mu.Set[Int]()
  dp(0) = 1
  (0 until n) foreach { i =>
    if (s(f(i))) {
      while (f(p) != f(i)) {
        s -= f(p)
        p += 1
      }
      s -= f(p)
      p += 1
    }
    assert (p <= i)
    assert (!s(f(i)))
    assert (s.size == i - p)
    dp(i + 1) = dp.query(p, i)
    s += f(i)
  }
  println(dp(n))
}
