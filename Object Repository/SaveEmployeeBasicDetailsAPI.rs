<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SaveEmployeeBasicDetailsAPI</name>
   <tag></tag>
   <elementGuidId>cf2f81d9-38db-4aea-9bc1-2c866df3999b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;cmpCode\&quot;: 1,\n  \&quot;empNum\&quot;: \&quot;REG2267\&quot;,\n  \&quot;epfNo\&quot;: \&quot;1234\&quot;,\n  \&quot;salutation\&quot;: \&quot;MR\&quot;,\n  \&quot;fstName\&quot;: \&quot;first\&quot;,\n  \&quot;midName\&quot;: \&quot;Middle\&quot;,\n  \&quot;lstName\&quot;: \&quot;last\&quot;,\n  \&quot;initials\&quot;: \&quot;A.A.\&quot;,\n  \&quot;fullNam\&quot;: \&quot;A.S.D.Prerea test1@4%#\&quot;,\n  \&quot;dob\&quot;: \&quot;1975-05-25\&quot;,\n  \&quot;gender\&quot;: \&quot;M\&quot;,\n  \&quot;deptmn\&quot;: \&quot;HR\&quot;,\n  \&quot;subDept\&quot;: \&quot;HR1\&quot;,\n  \&quot;section\&quot;: \&quot;S2\&quot;,\n  \&quot;empCat\&quot;: \&quot;PER\&quot;,\n  \&quot;catPrdYr\&quot;: 4,\n  \&quot;catPrdMo\&quot;:6,\n  \&quot;catPrdFrm\&quot;:\&quot;1900-05-27\&quot;,\n  \&quot;catPrdTo\&quot;:\&quot;2016-05-12\&quot;,\n  \&quot;desgn\&quot;:\&quot;SE\&quot;,\n  \&quot;jntDate\&quot;:\&quot;2012-01-05\&quot;,\n  \&quot;bassal\&quot;:\&quot;25800\&quot;,\n  \&quot;payCatType\&quot;:\&quot;SO\&quot;,\n  \&quot;status\&quot;:\&quot;A\&quot;,\n  \&quot;statusType\&quot;:\&quot;RET\&quot;,\n  \&quot;efDate\&quot;:\&quot;2012-03-12\&quot;,\n  \&quot;crtBy\&quot;:\&quot;RP\&quot;\n  }&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://localhost:8080/xeno/employee/save-basic-details</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
