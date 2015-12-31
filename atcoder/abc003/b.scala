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
  val s, t = sc.next
  val n = s.length
  def e(s: Char, t: Char): Boolean = {
    val q = "atcoder"
    s == t || (s == '@' && q.contains(t)) || (q.contains(s) && t == '@')
  }
  println (if ((0 until n).forall(i => e(s(i), t(i)))) "You can win" else "You will lose")
}