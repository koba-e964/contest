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
  
  def solve(s:String, t:String) : Boolean = {
    val bs = mu.BitSet()
    t foreach { i => bs(i - 'a') = true }
    val tr = s.filter((i : Char) => bs(i - 'a'))
    tr.contains(t)
  }
  
  val q = sc.nextInt
  (0 until q) foreach {_ =>
    val s,t = sc.next
    println(if (solve(s, t)) "YES" else "NO")
  }
}