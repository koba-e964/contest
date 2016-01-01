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
  val c = Array.fill(n)(sc.nextInt)
  val div = c.map(i => c.count(i % _ == 0))
  val sur = div.map(i => if (i % 2 == 0) 0.5 else (i + 1) / 2.0 / i)
  println(sur.sum)
}