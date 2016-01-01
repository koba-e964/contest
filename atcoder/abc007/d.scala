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
  @tailrec
  def banned(v: Long): Boolean =
    v % 10 == 4 || v % 10 == 9 || (v > 0 && banned(v / 10))
  /* [0, b) */
  def solve(b: Long): Long = {
    val tbl = Array(0,0,0,0,0,1,1,1,1,1)
    if (b == 0) 0
    else if (banned(b / 10))
      b / 10 * 2 + 8 * solve(b / 10) + b % 10
    else
      b / 10 * 2 + 8 * solve(b / 10) + tbl((b % 10).asInstanceOf[Int])      
  }
  val a, b = sc.nextLong
  println(solve(b + 1) - solve(a))
}