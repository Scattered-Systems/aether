job("Build and Push: Aether") {
    docker {
        build {
            context = "./code/crates"
            file = "./code/crates/Dockerfile"
            args["HTTP_PROXY"] = "http://0.0.0.0:8080"
            labels["vendor"] = "scattered-systems"
        }

        push("scattered-systems.registry.jetbrains.space/p/scsys/containers/aether") {
            tags("0.0.\$JB_SPACE_EXECUTION_NUMBER", "latest")
        }
    }
}