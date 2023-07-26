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

WebUI.navigateToUrl('https://thinking-tester-contact-list.herokuapp.com/')

WebUI.setText(findTestObject('Object Repository/Page_Contact List App/input_here_email'), 'cool.preeti12345@gmail.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Contact List App/input_here_password'), '7Dxoto1EjKfQbGop/Ov+1g==')

WebUI.click(findTestObject('Object Repository/Page_Contact List App/button_Submit'))

WebUI.click(findTestObject('Object Repository/Page_My Contacts/button_Add a New Contact'))

WebUI.setText(findTestObject('Object Repository/Page_Add Contact/input_First Name_firstName'), 'Abc')

WebUI.setText(findTestObject('Object Repository/Page_Add Contact/input_Last Name_lastName'), 'Kumar')

WebUI.setText(findTestObject('Object Repository/Page_Add Contact/input_Date of Birth_birthdate'), '2023-10-07')

WebUI.setText(findTestObject('Object Repository/Page_Add Contact/input_Email_email'), 'cool.preeti12345@gmail.com')

WebUI.setText(findTestObject('Object Repository/Page_Add Contact/input_Phone_phone'), '8765432342')

WebUI.setText(findTestObject('Object Repository/Page_Add Contact/input_Street Address 1_street1'), '123')

WebUI.setText(findTestObject('Object Repository/Page_Add Contact/input_Street Address 2_street2'), '123')

WebUI.setText(findTestObject('Object Repository/Page_Add Contact/input_City_city'), 'Test')

WebUI.setText(findTestObject('Object Repository/Page_Add Contact/input_State or Province_stateProvince'), 'Test12')

WebUI.setText(findTestObject('Object Repository/Page_Add Contact/input_Postal Code_postalCode'), '122003')

WebUI.setText(findTestObject('Object Repository/Page_Add Contact/input_Country_country'), 'India')

WebUI.click(findTestObject('Object Repository/Page_Add Contact/button_Submit'))

WebUI.click(findTestObject('Object Repository/Page_My Contacts/button_Logout'))

WebUI.closeBrowser()

