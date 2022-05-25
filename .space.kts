job("Build and Push: RustBox") {
    docker {
        build {
            context = "."
            file = "./Dockerfile"
            args["HTTP_PROXY"] = "http://0.0.0.0:8080"
            labels["vendor"] = "scattered-systems"
        }

        push("scattered-systems.registry.jetbrains.space/p/personal/containers/rustbox") {
            tags("0.0.\$JB_SPACE_EXECUTION_NUMBER", "latest")
        }
    }
}