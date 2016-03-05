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
  type P = (Double, Double)
  def cross(a: P, b: P) = (a, b) match {
    case ((p, q), (r, s)) => p * s - q * r
  }
  def m(a: P, b: P) = (a, b) match {
    case ((p, q), (r, s)) => (p - r, q - s)
  }
  def intersect(a: P, b: P, p: P, q: P): Boolean = {
    val eps = 1e-9
    cross(m(b,a), m(p,a)) * cross(m(b,a), m(q,a)) < eps &&
    cross(m(q,p), m(a,p)) * cross(m(q,p), m(b,p)) < eps
  }
  val sc = new ju.Scanner(System.in)
  val a, b = (sc.nextDouble, sc.nextDouble)
  val n = sc.nextInt
  val xys = Array.fill(n)((sc.nextDouble, sc.nextDouble))
  // if the edge p_i p_{i+1}'s crosses the segment ab 2m times, the answer is m + 1.
  val r = (0 until n).count {i => intersect(a, b, xys(i), xys((i + 1) % n))}
  assert (r % 2 == 0)
  println(r / 2 + 1)
}