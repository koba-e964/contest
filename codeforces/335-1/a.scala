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
  
  val n = sc.nextInt
  val a = Array.fill(n)(sc.nextInt - 1)
  val b = Array.fill(n + 1)(-1)
  (0 until n) foreach {i => b(a(i)) = i}
  var ma = 0
  var cur, start = 0
  (0 to n) foreach {i =>
    if (b(i) < cur) {
      ma = ma max (i - start)
      start = i
    }
    cur =  b(i)
  }
  println(n - ma)

}