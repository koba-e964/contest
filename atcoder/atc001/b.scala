class UnionFind(n: Int) {
  private[this] val disj = Array.tabulate(n)(x => x)
  def root(x: Int): Int =
    if (disj(x) != x) {
      disj(x) = root(disj(x))
      disj(x)
    } else x
  def unite(x: Int, y: Int) {
    disj(root(y)) = root(x)
  }
  def toArray: Array[Int] = disj.to
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
  val n, q = sc.nextInt
  val uf = new UnionFind(n)
  (0 until q) foreach {_ =>
    val p, a, b = sc.nextInt
    p match {
      case 0 => uf.unite(a, b)
      case 1 => println(if (uf.root(a) == uf.root(b)) "Yes" else "No")
    }
  }
}