/*
 * This is a code example for adding an object to a DynamoDB database
 * Using marshalling, rather than specifying all of our fields manually.
 */
package main

import (
	"context"
	"fmt"

	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/feature/dynamodb/attributevalue"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
)

// This is an example struct for adding data to our DynamoDB table
// Each property needs to start with a capital letter
// the id is our partition key
type A struct {
	Id     string `dynamodbav:"id"`
	Title  string `dynamodbav:"title"`
	Artist string `dynamodbav:"artist"`
}

func main() {

  // Load credentials from ENV variables or ~/.aws/credentials
	cfg, err := config.LoadDefaultConfig(context.TODO(), func(o *config.LoadOptions) error {
		o.Region = "us-east-1"
		return nil
	})

	if err != nil {
		panic(err)
	}

	svc := dynamodb.NewFromConfig(cfg)

  // Create a new item with a struct
	itemStruct := A{
		Id:     "123456",
		Title:  "Good Luck, Babe!",
		Artist: "Chappell Roan",
	}

  // Turn said item into a format the AWS SDK understands
	item, err := attributevalue.MarshalMap(itemStruct)

	if err != nil {
		panic(err)
	}

  // Put the item into the database
	out, err := svc.PutItem(context.TODO(), &dynamodb.PutItemInput{
		TableName: aws.String("efereira_Songs"), Item: item,
	})

	if err != nil {
		panic(err)
	}

  // Check that it worked!
	fmt.Println(out.Attributes)
}
