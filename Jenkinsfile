pipeline {
  agent { label 'UbuntuVM' }
  options { skipDefaultCheckout() }
  parameters {
    gitParameter(name: 'GIT_TAG',
                 type: 'PT_BRANCH_TAG',
                 description: 'The Git tag to checkout. If not specified "master" will be checkout.',
                 defaultValue: '0.5.0-beta.3')
    booleanParam(name: 'PUBLISH_RESULTS',
                 description: 'Publish the resulting artifacts (to Eclipse download)',
                 defaultValue: false)
  }
  environment {
      LABEL = get_label()
      MACOSX_DEPLOYMENT_TARGET=10.7
  }

  stages {
    stage('Checkout Git TAG') {
      agent { label 'MacMini' }
      steps {
        cleanWs()
        checkout([$class: 'GitSCM',
                  branches: [[name: "${params.GIT_TAG}"]],
                  doGenerateSubmoduleConfigurations: false,
                  extensions: [],
                  gitTool: 'Default',
                  submoduleCfg: [],
                  userRemoteConfigs: [[url: 'https://github.com/eclipse-zenoh/zenoh-c.git']]
                ])
      }
    }
    stage('MacOS Build') {
      agent { label 'MacMini' }
      steps {
        sh '''
        env
        make all
        '''
      }
    }
    stage('MacOS Package') {
      agent { label 'MacMini' }
      steps {
        sh '''
        tar -cvf eclipse-zenoh-c-${LABEL}-macosx${MACOSX_DEPLOYMENT_TARGET}-x86-64.tar --strip-components 2 target/release/*.dylib
        tar -rvf eclipse-zenoh-c-${LABEL}-macosx${MACOSX_DEPLOYMENT_TARGET}-x86-64.tar include
        gzip eclipse-zenoh-c-${LABEL}-macosx${MACOSX_DEPLOYMENT_TARGET}-x86-64.tar
        tar -czvf eclipse-zenoh-c-${LABEL}-examples-macosx${MACOSX_DEPLOYMENT_TARGET}-x86-64.tar.gz --exclude 'target/release/examples/*.*' --strip-components 3 target/release/examples/*
        '''
        stash includes: 'eclipse-zenoh-c-*-macosx*-x86-64.tar.gz', name: 'zenoh-c-MacOS'
      }
    }


    stage('x86_64-unknown-linux-gnu Build') {
      agent { label 'MacMini' }
      steps {
        sh '''
        docker run --init --rm -v $(pwd):/workdir -w /workdir --env "TARGET=x86_64-unknown-linux-gnu" \
            adlinktech/manylinux2010-x64-rust-nightly make all
        '''
      }
    }
    stage('x86_64-unknown-linux-gnu Package') {
      agent { label 'MacMini' }
      steps {
        sh '''
        tar -cvf eclipse-zenoh-c-${LABEL}-x86_64-unknown-linux-gnu.tar --strip-components 3 target/x86_64-unknown-linux-gnu/release/*.so
        tar -rvf eclipse-zenoh-c-${LABEL}-x86_64-unknown-linux-gnu.tar include
        gzip eclipse-zenoh-c-${LABEL}-x86_64-unknown-linux-gnu.tar
        tar -czvf eclipse-zenoh-c-${LABEL}-examples-x86_64-unknown-linux-gnu.tar.gz --exclude 'target/x86_64-unknown-linux-gnu/release/examples/*.*' --exclude 'target/x86_64-unknown-linux-gnu/release/examples/*-*' --strip-components 4 target/x86_64-unknown-linux-gnu/release/examples/*
        '''
        stash includes: 'eclipse-zenoh-c-*-x86_64-unknown-linux-gnu.tar.gz', name: 'zenoh-c-Linux-x64'
      }
    }

    stage('i686-unknown-linux-gnu Build') {
      agent { label 'MacMini' }
      steps {
        sh '''
        docker run --init --rm -v $(pwd):/workdir -w /workdir --env "TARGET=i686-unknown-linux-gnu" \
            adlinktech/manylinux2010-i686-rust-nightly make all
        '''
      }
    }
    stage('i686-unknown-linux-gnu Package') {
      agent { label 'MacMini' }
      steps {
        sh '''
        tar -cvf eclipse-zenoh-c-${LABEL}-i686-unknown-linux-gnu.tar --strip-components 3 target/i686-unknown-linux-gnu/release/*.so
        tar -rvf eclipse-zenoh-c-${LABEL}-i686-unknown-linux-gnu.tar include
        gzip eclipse-zenoh-c-${LABEL}-i686-unknown-linux-gnu.tar
        tar -czvf eclipse-zenoh-c-${LABEL}-examples-i686-unknown-linux-gnu.tar.gz --exclude 'target/i686-unknown-linux-gnu/release/examples/*.*' --exclude 'target/i686-unknown-linux-gnu/release/examples/*-*' --strip-components 4 target/i686-unknown-linux-gnu/release/examples/*
        '''
        stash includes: 'eclipse-zenoh-c-*-i686-unknown-linux-gnu.tar.gz', name: 'zenoh-c-Linux-i686'
      }
    }

    stage('Publish to download.eclipse.org') {
      steps {
        // Unstash packages to be deployed
        unstash 'zenoh-c-MacOS'
        unstash 'zenoh-c-Linux-x64'
        unstash 'zenoh-c-Linux-i686'
        sshagent ( ['projects-storage.eclipse.org-bot-ssh']) {
          sh '''
          if [ "${PUBLISH_RESULTS}" = "true" ]; then
            export TARGET_DIR=/home/data/httpd/download.eclipse.org/zenoh/zenoh/${LABEL}/zenoh-c
            ssh genie.zenoh@projects-storage.eclipse.org mkdir -p ${TARGET_DIR}
            ssh genie.zenoh@projects-storage.eclipse.org ls -al ${TARGET_DIR}
            scp eclipse-zenoh-c-${LABEL}-*.tar.gz genie.zenoh@projects-storage.eclipse.org:${TARGET_DIR}
          else
            echo "Publication to download.eclipse.org skipped"
          fi
          '''
        }
      }
    }
  }
}

def get_label() {
    return env.GIT_TAG.startsWith('origin/') ? env.GIT_TAG.minus('origin/') : env.GIT_TAG
}
