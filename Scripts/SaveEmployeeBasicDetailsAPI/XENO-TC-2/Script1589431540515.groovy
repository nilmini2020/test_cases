/* This is post method test cases for API*/


import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent
import java.util.List
import java.util.concurrent.locks.Condition
import java.util.ArrayList
import com.kms.katalon.core.testobject.ConditionType

//POST Object

def request=(RequestObject)findTestObject("SaveEmployeeBasicDetailsAPI")

String body = '{"cmpCode": 1,"empNum": "REG2288","epfNo": "1234","salutation": "MR","fstName": "first","midName": "Middle","lstName": "last","initials": "A.A.","fullNam": "A.S.D.Prerea ","dob": "1975-05-25","gender": "M","deptmn": "HR","subDept": "HR1","section": "S2","empCat": "PER","catPrdYr": 4,"catPrdMo":6,"catPrdFrm":"1900-05-27","catPrdTo":"2016-05-12","desgn":"SE","jntDate":"2012-01-05","bassal":"25800","payCatType":"SO","status":"A","statusType":"RET","efDate":"2012-03-12","crtBy":"RP"}'

try{
	
	request.setBodyContent(new HttpTextBodyContent(body,"UTF-8","application/json"))
}
  catch(Exception ex)
  {
	  println ex
  }
  WS.sendRequest(request)
  
  //GET response if there is a get method for save details we can use
/*
  def response=(RequestObject)findTestObject("GET")
  
  List<TestObjectProperty> params=new ArrayList();
  params.add(new TestObjectProperty('cmpCode',ConditionType.equals,"2"))
  response.setRestParameters(params)
  
  def result=WS.sendRequest(response)
  WS.verifyElementPropertyValue(result, "empNum","REG2297", FailureHandling.STOP_ON_FAILURE)
  */
