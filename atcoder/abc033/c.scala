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

  def needZero(s: String): Boolean = !s.contains('0')
  val sc = new ju.Scanner(System.in)
  val s = sc.next.split("\\+")
  println(s.filter(needZero).size)
  
}