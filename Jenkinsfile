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
          steps {
            echo 'Build 2 message'
            echo 'Build 2 - message 2'
            waitUntil(quiet: true, initialRecurrencePeriod: 2) {
              sh 'uptime'
            }

          }
        }

      }
    }

  }
}