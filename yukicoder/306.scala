object Main extends App {
  import java.{util => ju}
  import scala.annotation._
  import scala.collection._
  import scala.collection.{mutable => mu}
  import scala.collection.JavaConverters._
  import scala.math._

  val sc = new ju.Scanner(System.in)
  val a,b,c,d = sc.nextDouble
  val ans = (a * d + b * c) / (a + c)
  println(ans)
}