import Dependencies._

ThisBuild / scalaVersion     := "2.13.16"
ThisBuild / version          := "0.1.0-SNAPSHOT"
ThisBuild / organization     := "com.example"
ThisBuild / organizationName := "example"

lazy val root = (project in file("."))
  .enablePlugins(NativeImagePlugin) // added for graal vm 
  .settings(
    name := "learn-about-compilers-scala",
    Compile / mainClass := Some("tac.Main"),
    nativeImageCommand := Seq("/home/generic/.sdkman/candidates/java/21.0.2-graalce/bin/native-image"),
    libraryDependencies ++= Seq(
      munit % Test,
      "com.lihaoyi" %% "upickle" % "3.3.1",
      "com.lihaoyi" %% "os-lib" % "0.10.0",
      "org.graalvm.nativeimage" % "native-image-base" % "23.1.0"  % "provided"// added for graal vm 
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

