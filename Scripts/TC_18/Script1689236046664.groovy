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
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.telerik.com/support/demos')

WebUI.click(findTestObject('Object Repository/Page_Telerik Product Demos, Examples and Tu_d36ce2/a_Web'))

WebUI.click(findTestObject('Object Repository/Page_Telerik Product Demos, Examples and Tu_d36ce2/a_Desktop'))

WebUI.click(findTestObject('Object Repository/Page_Telerik Product Demos, Examples and Tu_d36ce2/a_Mobile'))

WebUI.click(findTestObject('Object Repository/Page_Telerik Product Demos, Examples and Tu_d36ce2/a_Reporting  QA'))

WebUI.click(findTestObject('Object Repository/Page_Telerik Product Demos, Examples and Tu_d36ce2/a_Conversational UI'))

WebUI.click(findTestObject('Object Repository/Page_Telerik Product Demos, Examples and Tu_d36ce2/span_Kendo UI for jQuery'))

WebUI.click(findTestObject('Object Repository/Page_Demo of core features in jQuery Chat w_c8f9c8/div_Components                             _bb3354'))

WebUI.closeBrowser()

