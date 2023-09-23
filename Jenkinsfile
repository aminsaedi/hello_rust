pipeline {
  agent any
  stages {
    stage('Check code format') {
      steps {
        sh 'echo "Checking code format and linters"'
      }
    }

    stage('Build State') {
      parallel {
        stage('Build State') {
          agent any
          steps {
            echo 'Building app'
          }
        }

        stage('Build 2') {
          agent any
          steps {
            echo 'Build 2 message'
            echo 'Build 2 - message 2'
          }
        }

      }
    }

  }
}