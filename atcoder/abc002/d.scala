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
  val xy = Array.fill(m)((sc.nextInt - 1, sc.nextInt - 1))
  val edge = Array.tabulate(n)(i => 1 << i)
  for ((x, y) <- xy) {
    edge(x) |= 1 << y
    edge(y) |= 1 << x
  }
  def isClique(s: Int): Boolean = {
    (0 until n).filter(i => (s & (1 << i)) != 0).forall(i => (edge(i) & s) == s)
  }
  println((0 until (1 << n)).filter(isClique(_)).map(Integer.bitCount(_)).max)
}