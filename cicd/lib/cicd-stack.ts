import * as cdk from 'aws-cdk-lib';
import {Construct} from 'constructs';
// import * as sqs from 'aws-cdk-lib/aws-sqs';
import * as dynamodb from 'aws-cdk-lib/aws-dynamodb';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as apigw from 'aws-cdk-lib/aws-apigateway';

export class CicdStack extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        // The code that defines your stack goes here

        // example resource
        // const queue = new sqs.Queue(this, 'CicdQueue', {
        //   visibilityTimeout: cdk.Duration.seconds(300)
        // });

        // dynamodb resource
        const table = new dynamodb.Table(this, 'TodoTable', {
            partitionKey: {name: 'id', type: dynamodb.AttributeType.STRING},
        });

        // lambda resource
        const fn = new lambda.Function(this, 'TodoFunction', {
            runtime: lambda.Runtime.PROVIDED_AL2,
            architecture: lambda.Architecture.X86_64,
            code: lambda.Code.fromAsset('../build'),
            handler: 'bootstrap',
            environment: {
                'TODO_TABLE_NAME': table.tableName,
            }
        });
        table.grantReadWriteData(fn);

        const api = new apigw.RestApi(this, 'Api');
        const todos = api.root.addResource("todos");
        todos.addMethod('GET', new apigw.LambdaIntegration(fn));
        todos.addMethod('POST', new apigw.LambdaIntegration(fn));
        //put
        const todo = todos.addResource("{id}");
        todo.addMethod('GET', new apigw.LambdaIntegration(fn));
        todo.addMethod('PUT', new apigw.LambdaIntegration(fn));





    }
}
