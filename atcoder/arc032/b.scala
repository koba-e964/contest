/**
 * Union-Find tree.
 * toArray returns an array consisting of the roots of all vertices.
 * Verified by: AtCoder ARC032-B(http://arc032.contest.atcoder.jp/submissions/603918)
 */
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
  def toArray: Array[Int] = {
    for (i <- 0 until n) root(i)
    disj.toArray
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
  val uf = new UnionFind(n)
  for (_ <- 0 until m) {
    val a, b = sc.nextInt - 1
    uf.unite(a, b)
  }
  println(uf.toArray.toSet.size - 1)
}