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
  val n, m = sc.nextInt
  val e: Array[(Int, Int)]= Array.fill(m)((sc.nextInt-1, sc.nextInt-1))
  val uf = new UnionFind(n)
  e foreach {case (u, v) => uf.unite(u, v)}
  val ne: Array[Int] = Array.fill(n)(0)
  val nv: Array[Int] = Array.fill(n)(0)
  e foreach {case (u : Int, v: Int) => ne(uf.root(u)) += 1}
  (0 until n) foreach {v => nv(uf.root(v)) += 1}
  println((0 until n).count(i => nv(i) == ne(i) + 1))
}