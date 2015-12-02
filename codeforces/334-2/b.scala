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
  val n, k = sc.nextInt
  val s = Array.fill(n)(sc.nextInt)
  val twos = 0 max (n - k)
  0 until twos foreach {i => s(i) += s(2 * twos - i - 1)}
  println(s.max)
}