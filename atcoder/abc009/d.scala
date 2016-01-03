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
  def mul(n: Int, a: Array[Long], b: Array[Long], m: Array[Long]): Array[Long] = {
    val ret = Array.fill(2 * n)(0L)
    for (i <- 0 until n) {
      for (j <- 0 until n) {
        ret(i + j) ^= a(i) & b(j)
      }
    }
    for (i <- (n - 1) to 0 by -1) {
      for (j <- 0 until n) {
        ret(i + j) ^= ret(i + n) & m(j)
      }
    }
    ju.Arrays.copyOfRange(ret, 0, n)
  }
  val k, m = sc.nextInt
  val a = Array.fill(k)(sc.nextLong)
  val c = Array.fill(k)(sc.nextLong).reverse
  var sum, cur = Array.fill(k)(0L)
  sum(0) = 0xffffffffL
  if (k >= 2) {
    cur(1) = 0xffffffffL
  } else {
    cur(0) = c(0)
  }
  var exp = m - 1
  while (exp > 0) {
    if (exp % 2 == 1) {
      sum = mul(k, sum, cur, c)
    }
    cur = mul(k, cur, cur, c)
    exp /= 2
  }
  println(a.zip(sum).map{case (x, y) => x & y}.foldLeft(0L)(_ ^ _))
}