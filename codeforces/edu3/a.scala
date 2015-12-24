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
  val n,m = sc.nextInt
  val a = Array.fill(n)(sc.nextInt).sortWith(_ > _).inits.map(_.sum)
  println(n + 1 - a.count(_ >= m))
  
  
}