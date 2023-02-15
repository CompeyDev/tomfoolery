use tensorflow::{Status, Code, Tensor, Graph, SavedModelBundle, SessionOptions, SessionRunArgs};
use rand::random;

#[cfg_attr(feature = "examples_system_alloc", global_allocator)]
#[cfg(feature = "examples_system_alloc")]
static ALLOCATOR: std::alloc::System = std::alloc::System;

pub fn train() -> Result<(), Box<dyn Error>> {
    const DATABASE_LOCATION: &'static str = "./model.pb";

    if !Path::new(DATABASE_LOCATION).exists() {
        return Err(
            Box::new(
                Status::new_set(
                    Code::NotFound, 
                    &format!("Model database file not located, please create a \`model.pb\` file!")
                )
                .unwrap(),
            )
        );
    }

    let w = 0.1;
    let b = 0.3;

    let num_plots = 100;
    let steps = 201;

    let mut x = Tensor::new(&[num_plots as u64]);
    let mut y = Tensor::new(&[num_plots as u64]);

    for i in 0..num_plots {
        x[i] = (2.0 * random::<f64>() - 1.0) as f32;
        y[i] = w * x[i] + b;
    }

    let mut graph = Graph::new();
    let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, "data/training")?;
    let session = &bundle.session;

    let signature = bundle.meta_graph_def().get_signature("train")?;
    let x_data = signature.get_input("x")?;
    let y_data = signature.get_input("y")?;
    let loss_data = signature.get_output("loss")?;
    let op_x = graph.operation_by_name_required(&x_data.name().name)?;
    let op_y = graph.operation_by_name_required(&y_data.name().name)?;
    let op_train = graph.operation_by_name_required(&loss_data.name().name)?;

    let op_b = { 
        let b_signature = bundle.meta_graph_def().get_signature("b")?;
        let b_data = b_signature.get_output("output")?;

        graph.operation_by_name_required(&b_data.name().name)?
    };

    let op_w = { 
        let w_signature = bundle.meta_graph_def().get_signature("w")?;
        let w_data = b_signature.get_output("output")?;

        graph.operation_by_name_required(&w_data.name().name)?
    };

    let mut training_step = SessionRunArgs::new();
    training_step.add_feed(&op_x, 0, &x);
    training_step.add_feed(&op_y, 0, &y);
    training_step.add_target(&op_train);

    for _ in 0..steps {
        session.run(&mut training_step)?;
    }

    let mut output_step = SessionRunArgs::new();
    let w_ix = output_step.request_fetch(&op_w, 0);
    let b_ix = output_step.request_fetch(&op_b, 0);
    session.run(&mut output_step);

    let w_hat: f32 = output_step.fetch(w_ix)?[0];
    let b_hat: f32 = output_step.fetch((b_ix))?[0];
    
    println!("training :: w -> expected {w}, received {w_hat}; {}!", (w - w_hat <= 0.1));
    println!("training :: b -> expected {b}, received {b_hat}; {}!", (b - b_hat <= 0.1));


    Ok(())
}