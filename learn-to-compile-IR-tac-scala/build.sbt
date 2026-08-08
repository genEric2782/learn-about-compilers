import Dependencies._
import scala.util.Try
import scala.sys.process._

ThisBuild / scalaVersion     := "2.13.16"
ThisBuild / version          := "0.1.0-SNAPSHOT"
ThisBuild / organization     := "com.example"
ThisBuild / organizationName := "example"

// Resolve native-image without hardcoding a path/user.
// Priority: NATIVE_IMAGE_CMD env var > $JAVA_HOME/bin/native-image > PATH lookup.
lazy val nativeImagePath: String = {
  import scala.sys.process._
  sys.env.get("NATIVE_IMAGE_CMD").filter(p => new java.io.File(p).exists()) match {
    case Some(p) => p
    case None =>
      val fromJavaHome = sys.env.get("JAVA_HOME").map(h => new java.io.File(h, "bin/native-image"))
      fromJavaHome.filter(_.exists()) match {
        case Some(f) => f.getAbsolutePath
        case None =>
          // fall back to whatever's on PATH
          Try("which native-image".!!.trim).toOption.filter(_.nonEmpty).getOrElse(
            sys.error(
              "Could not locate 'native-image'. Set NATIVE_IMAGE_CMD or JAVA_HOME " +
              "to a GraalVM install with the native-image component (run `gu install native-image`)."
            )
          )
      }
  }
}

lazy val root = (project in file("."))
  .enablePlugins(NativeImagePlugin) // added for graal vm 
  .settings(
    name := "learn-about-compilers-scala",
    Compile / mainClass := Some("tac.Main"),
    nativeImageCommand := Seq(nativeImagePath),
    libraryDependencies ++= Seq(
      munit % Test,
      "com.lihaoyi" %% "upickle" % "3.3.1",
      "com.lihaoyi" %% "os-lib" % "0.10.0",
      "org.graalvm.nativeimage" % "native-image-base" % "23.1.0" % "provided"// added for graal vm 
    ), 
    // added for grall vm 
    nativeImageOptions ++= Seq(
      "--no-fallback",
      "--shared",
      "-H:+ReportExceptionStackTraces",
      "-H:+AllowDeprecatedBuilderClassesOnImageClasspath",
      "-H:Name=liblearn-about-compilers-scala",
      "-H:Class=tac.Main$"   // points to the actual JVM class name for a Scala object
    )
  )

// See https://www.scala-sbt.org/1.x/docs/Using-Sonatype.html for instructions on how to publish to Sonatype.

