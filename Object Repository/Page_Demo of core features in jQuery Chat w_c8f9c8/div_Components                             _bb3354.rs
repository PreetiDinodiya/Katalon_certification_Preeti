<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Components                             _bb3354</name>
   <tag></tag>
   <elementGuidId>526ab76f-44ac-4026-bfab-4f6834a7c470</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Keyboard Navigation'])[60]/following::div[2]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.container.container-sm</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>cac6c7fd-b56f-4d9b-8dcd-b34a433c8abc</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>container container-sm</value>
      <webElementGuid>d06915d5-c0aa-4119-a9b7-f6dd2a2fcb9b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                

                
                    
                        
                            Components
                            / 
                        
                        
                            Chat
                            / 
                        
                        
                            Travel
                        
                    
                

                

                
                
jQuery
Chat
Travel                

                
                


    
        
            .a{fill:#a2d7f1;}
        
        Run Live Demo
    

    
        
            
                .a{fill:#a2d7f1;}
            
            Loading Demo...
        
    

    
        
            
                EXAMPLE
            
            
                VIEW SOURCE
            
            
            
                
                    
                        Edit in Kendo UI Dojo
                    
                    
                        





                    
                
            
            
            

    window.selectedTheme = &quot;default-ocean-blue&quot;;
    window.selectedThemeCommon = &quot;common-empty&quot;;


Change Theme

    
            

            
            
            


            
                default
            
        
        
            






        
    
    
        
                    
                        
                            Default Theme
                        
                        
                                
                                    

            
            
            


                                    Main
                                
                                
                                    

            
            
            


                                    Main Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Nordic
                                
                                
                                    

            
            
            


                                    Ocean Blue
                                
                                
                                    

            
            
            


                                    Ocean Blue A11Y
                                        
                                            
  accessibility
  


                                        
                                
                                
                                    

            
            
            


                                    Purple
                                
                                
                                    

            
            
            


                                    Turquoise
                                
                        
                    
                    
                        
                            Bootstrap Theme
                        
                        
                                
                                    

            
            
            


                                    Main
                                
                                
                                    

            
            
            


                                    Bootstrap 3
                                
                                
                                    

            
            
            


                                    Bootstrap 3 Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Main Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Nordic
                                
                                
                                    

            
            
            


                                    Turquoise
                                
                                
                                    

            
            
            


                                    Turquoise Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Urban
                                
                                
                                    

            
            
            


                                    Vintage
                                
                        
                    
                    
                        
                            Material Theme
                        
                        
                                
                                    

            
            
            


                                    Main
                                
                                
                                    

            
            
            


                                    Arctic
                                
                                
                                    

            
            
            


                                    Lime Dark
                                
                                
                                    

            
            
            


                                    Main Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Nova
                                
                        
                    
                    
                        
                            Classic Theme
                        
                        
                                
                                    

            
            
            


                                    Main
                                
                                
                                    

            
            
            


                                    Green
                                
                                
                                    

            
            
            


                                    Green Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Lavender
                                
                                
                                    

            
            
            


                                    Lavender Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Main Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Metro
                                
                                
                                    

            
            
            


                                    Metro Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Moonlight
                                
                                
                                    

            
            
            


                                    Opal
                                
                                
                                    

            
            
            


                                    Opal Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Silver
                                
                                
                                    

            
            
            


                                    Silver Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Uniform
                                
                        
                    
                    
                        
                            Fluent Theme
                        
                        
                                
                                    

            
            
            


                                    Main
                                
                        
                    
        
        
            
                
                    



	
		
	
































                
                Launch Theme Builder
            
        
    

        
    
    
                    
    
        html .k-chat .k-card-deck .k-card {
            flex-basis: 230px;
        }

        .k-card > img.k-card-image {
            height: 115px;
            display: block;
        }
    

    
        Type a message...
    

    
    

    
    

    
        $.when(
            $.getScript(&quot;https://unpkg.com/botframework-directlinejs@0.11.5/dist/directline.js&quot;),
            $.getScript(&quot;https://unpkg.com/adaptivecards@2.10.0/dist/adaptivecards.min.js&quot;)
        ).done(function () {
            window.agent = new DirectLineAgent(chat, &quot;Y_ly-If6haE.cwA.PQE.ZwOOsq4MlHcD3_YLFI-t9oW6L6DXMMBoi67LBz9WaWA&quot;);
        });

        window.DirectLineAgent = kendo.Class.extend({
            init: function (chat, secret) {
                this.chat = chat;
                this.iconUrl = &quot;../content/chat/VacationBot.png&quot;;

                this.agent = new DirectLine.DirectLine({ secret: secret });

                this.agent.activity$.subscribe($.proxy(this.onResponse, this));
            },

            postMessage: function (args) {
                var postArgs = {
                    text: args.text,
                    type: args.type,
                    timestamp: args.timestamp,
                    from: args.from
                };

                this.agent.postActivity(postArgs)
                    .subscribe();
            },

            onResponse: function (response) {
                if (response.from.id === this.chat.getUser().id) {
                    return;
                }

                response.from.iconUrl = this.iconUrl;

                this.renderMessage(response);

                this.renderAttachments(response);

                this.renderSuggestedActions(response.suggestedActions);
            },

            renderMessage: function (message) {
                if (message.text || message.type == &quot;typing&quot;) {
                    this.chat.renderMessage(message, message.from);
                }
            },

            renderAttachments: function (data) {
                this.chat.renderAttachments(data, data.from);
            },

            renderSuggestedActions: function (suggestedActions) {
                var actions = [];

                if (suggestedActions &amp;&amp; suggestedActions.actions) {
                    actions = suggestedActions.actions;
                }

                this.chat.renderSuggestedActions(actions);
            }
        });

        var chat = $(&quot;#chat&quot;).kendoChat({
            post: function (args) {
                agent.postMessage(args);
            }
        }).data(&quot;kendoChat&quot;);

        var AdaptiveCardComponent = kendo.chat.Component.extend({
            init: function (options, view) {
                kendo.chat.Component.fn.init.call(this, options, view);

                var adaptiveCard = new AdaptiveCards.AdaptiveCard();

                adaptiveCard.hostConfig = new AdaptiveCards.HostConfig({
                    fontFamily: &quot;Segoe UI, Helvetica Neue, sans-serif&quot;
                });

                adaptiveCard.parse(options);

                var bodyElement = $(&quot;&lt;div>&quot;).addClass(&quot;k-card-body&quot;).append(adaptiveCard.render());
                this.element.addClass(&quot;k-card&quot;).append(bodyElement);
            }
        });

        kendo.chat.registerComponent(&quot;application/vnd.microsoft.card.adaptive&quot;, AdaptiveCardComponent);

    


                
    
        
    
        
            
                    
                        index.html
                    
            
        
            
    

    






    
        Also available for:
        
            
                    ASP.NET MVC
                    ASP.NET Core
                    JSP
                    PHP
            
        
        
            
                API REFERENCE
                
                    





                
            
    



                
                    &lt;div id=&quot;example&quot;>
    &lt;style>
        html .k-chat .k-card-deck .k-card {
            flex-basis: 230px;
        }

        .k-card > img.k-card-image {
            height: 115px;
            display: block;
        }
    &lt;/style>

    &lt;div class=&quot;demo-section&quot;>
        &lt;div id=&quot;chat&quot;>&lt;/div>
    &lt;/div>

    &lt;!-- Load Bot Framework Client API -->
    &lt;script src=&quot;https://unpkg.com/botframework-directlinejs@0.11.5/dist/directline.js&quot;>&lt;/script>

    &lt;!-- Load Adaptive Cards Client API -->
    &lt;script src=&quot;https://unpkg.com/adaptivecards@2.10.0/dist/adaptivecards.min.js&quot;>&lt;/script>

    &lt;script>
        $.when(
            $.getScript(&quot;https://unpkg.com/botframework-directlinejs@0.11.5/dist/directline.js&quot;),
            $.getScript(&quot;https://unpkg.com/adaptivecards@2.10.0/dist/adaptivecards.min.js&quot;)
        ).done(function () {
            window.agent = new DirectLineAgent(chat, &quot;Y_ly-If6haE.cwA.PQE.ZwOOsq4MlHcD3_YLFI-t9oW6L6DXMMBoi67LBz9WaWA&quot;);
        });

        window.DirectLineAgent = kendo.Class.extend({
            init: function (chat, secret) {
                this.chat = chat;
                this.iconUrl = &quot;../content/chat/VacationBot.png&quot;;

                this.agent = new DirectLine.DirectLine({ secret: secret });

                this.agent.activity$.subscribe($.proxy(this.onResponse, this));
            },

            postMessage: function (args) {
                var postArgs = {
                    text: args.text,
                    type: args.type,
                    timestamp: args.timestamp,
                    from: args.from
                };

                this.agent.postActivity(postArgs)
                    .subscribe();
            },

            onResponse: function (response) {
                if (response.from.id === this.chat.getUser().id) {
                    return;
                }

                response.from.iconUrl = this.iconUrl;

                this.renderMessage(response);

                this.renderAttachments(response);

                this.renderSuggestedActions(response.suggestedActions);
            },

            renderMessage: function (message) {
                if (message.text || message.type == &quot;typing&quot;) {
                    this.chat.renderMessage(message, message.from);
                }
            },

            renderAttachments: function (data) {
                this.chat.renderAttachments(data, data.from);
            },

            renderSuggestedActions: function (suggestedActions) {
                var actions = [];

                if (suggestedActions &amp;&amp; suggestedActions.actions) {
                    actions = suggestedActions.actions;
                }

                this.chat.renderSuggestedActions(actions);
            }
        });

        var chat = $(&quot;#chat&quot;).kendoChat({
            post: function (args) {
                agent.postMessage(args);
            }
        }).data(&quot;kendoChat&quot;);

        var AdaptiveCardComponent = kendo.chat.Component.extend({
            init: function (options, view) {
                kendo.chat.Component.fn.init.call(this, options, view);

                var adaptiveCard = new AdaptiveCards.AdaptiveCard();

                adaptiveCard.hostConfig = new AdaptiveCards.HostConfig({
                    fontFamily: &quot;Segoe UI, Helvetica Neue, sans-serif&quot;
                });

                adaptiveCard.parse(options);

                var bodyElement = $(&quot;&lt;div>&quot;).addClass(&quot;k-card-body&quot;).append(adaptiveCard.render());
                this.element.addClass(&quot;k-card&quot;).append(bodyElement);
            }
        });

        kendo.chat.registerComponent(&quot;application/vnd.microsoft.card.adaptive&quot;, AdaptiveCardComponent);

    &lt;/script>
&lt;/div>

                

                
                    
                        
                            
                                
                                    
                                        
                                    
                                    
                                        
                                            
                                                The Chat component is part of Kendo UI for jQuery, a professional grade UI library with 110+ components for building modern and feature-rich applications. To try it out sign up for a free 30-day trial.
                                            
                                        
                                    
                                
                                
                                    Download Free Trial
                                
                            
                        
                    

                
            

            
                Description
                
                    The Kendo UI Chat widget allows for integration with any bot framework, due to its simplicity, flexible API and customizable templates.This demo demonstrates how the widget can be connected to a chat bot created in Microsoft's Bot Framework, using the DirectLineJS client library. The sample shows off out of the box features of the Chat widget, such as the hero cards and suggested actions, as well as the ability to define custom components, which let you use javascript to render any content. In this particular case we use the AdaptiveCards client API to render the adaptive cards returned by the service.
                
            
            
            
                Support &amp; Learning Resources
                
                        jQuery Chat Documentation Overview
                        jQuery Chat API
                        jQuery Chat Forums
                        Kendo UI for jQuery Chat Product Homepage
                    Kendo UI for jQuery Knowledge Base
                
            

                
                    Additional Resources
                    
                        Kendo UI for jQuery Product Overview
                        Kendo UI for jQuery Blog
                        Kendo UI for jQuery Videos
                        Kendo UI for jQuery Roadmap
                        Kendo UI for jQuery Pricing
                        Kendo UI for jQuery Training
                    
                
        
    </value>
      <webElementGuid>016ea4ae-d5e4-4cb5-8389-52d563a517ef</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;k-default-ocean-blue kd-mode k-webkit k-webkit114&quot;]/body[1]/main[@class=&quot;kd-jquery&quot;]/div[@class=&quot;kd-content-container&quot;]/div[@class=&quot;kd-demo-content&quot;]/div[@class=&quot;container container-sm&quot;]</value>
      <webElementGuid>cbd25d55-c4b1-4891-98eb-5a18c3c286fc</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Keyboard Navigation'])[60]/following::div[2]</value>
      <webElementGuid>c8427b93-51a6-4fd5-99fd-14d52ba6456c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Events'])[76]/following::div[2]</value>
      <webElementGuid>5c9dc7a7-ebb3-4cf3-a208-35a4cb102caa</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//main/div/div/div</value>
      <webElementGuid>06e55320-1211-491e-a959-2d18b43f0701</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
                

                
                    
                        
                            Components
                            / 
                        
                        
                            Chat
                            / 
                        
                        
                            Travel
                        
                    
                

                

                
                
jQuery
Chat
Travel                

                
                


    
        
            .a{fill:#a2d7f1;}
        
        Run Live Demo
    

    
        
            
                .a{fill:#a2d7f1;}
            
            Loading Demo...
        
    

    
        
            
                EXAMPLE
            
            
                VIEW SOURCE
            
            
            
                
                    
                        Edit in Kendo UI Dojo
                    
                    
                        





                    
                
            
            
            

    window.selectedTheme = &quot;default-ocean-blue&quot;;
    window.selectedThemeCommon = &quot;common-empty&quot;;


Change Theme

    
            

            
            
            


            
                default
            
        
        
            






        
    
    
        
                    
                        
                            Default Theme
                        
                        
                                
                                    

            
            
            


                                    Main
                                
                                
                                    

            
            
            


                                    Main Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Nordic
                                
                                
                                    

            
            
            


                                    Ocean Blue
                                
                                
                                    

            
            
            


                                    Ocean Blue A11Y
                                        
                                            
  accessibility
  


                                        
                                
                                
                                    

            
            
            


                                    Purple
                                
                                
                                    

            
            
            


                                    Turquoise
                                
                        
                    
                    
                        
                            Bootstrap Theme
                        
                        
                                
                                    

            
            
            


                                    Main
                                
                                
                                    

            
            
            


                                    Bootstrap 3
                                
                                
                                    

            
            
            


                                    Bootstrap 3 Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Main Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Nordic
                                
                                
                                    

            
            
            


                                    Turquoise
                                
                                
                                    

            
            
            


                                    Turquoise Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Urban
                                
                                
                                    

            
            
            


                                    Vintage
                                
                        
                    
                    
                        
                            Material Theme
                        
                        
                                
                                    

            
            
            


                                    Main
                                
                                
                                    

            
            
            


                                    Arctic
                                
                                
                                    

            
            
            


                                    Lime Dark
                                
                                
                                    

            
            
            


                                    Main Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Nova
                                
                        
                    
                    
                        
                            Classic Theme
                        
                        
                                
                                    

            
            
            


                                    Main
                                
                                
                                    

            
            
            


                                    Green
                                
                                
                                    

            
            
            


                                    Green Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Lavender
                                
                                
                                    

            
            
            


                                    Lavender Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Main Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Metro
                                
                                
                                    

            
            
            


                                    Metro Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Moonlight
                                
                                
                                    

            
            
            


                                    Opal
                                
                                
                                    

            
            
            


                                    Opal Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Silver
                                
                                
                                    

            
            
            


                                    Silver Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Uniform
                                
                        
                    
                    
                        
                            Fluent Theme
                        
                        
                                
                                    

            
            
            


                                    Main
                                
                        
                    
        
        
            
                
                    



	
		
	
































                
                Launch Theme Builder
            
        
    

        
    
    
                    
    
        html .k-chat .k-card-deck .k-card {
            flex-basis: 230px;
        }

        .k-card > img.k-card-image {
            height: 115px;
            display: block;
        }
    

    
        Type a message...
    

    
    

    
    

    
        $.when(
            $.getScript(&quot;https://unpkg.com/botframework-directlinejs@0.11.5/dist/directline.js&quot;),
            $.getScript(&quot;https://unpkg.com/adaptivecards@2.10.0/dist/adaptivecards.min.js&quot;)
        ).done(function () {
            window.agent = new DirectLineAgent(chat, &quot;Y_ly-If6haE.cwA.PQE.ZwOOsq4MlHcD3_YLFI-t9oW6L6DXMMBoi67LBz9WaWA&quot;);
        });

        window.DirectLineAgent = kendo.Class.extend({
            init: function (chat, secret) {
                this.chat = chat;
                this.iconUrl = &quot;../content/chat/VacationBot.png&quot;;

                this.agent = new DirectLine.DirectLine({ secret: secret });

                this.agent.activity$.subscribe($.proxy(this.onResponse, this));
            },

            postMessage: function (args) {
                var postArgs = {
                    text: args.text,
                    type: args.type,
                    timestamp: args.timestamp,
                    from: args.from
                };

                this.agent.postActivity(postArgs)
                    .subscribe();
            },

            onResponse: function (response) {
                if (response.from.id === this.chat.getUser().id) {
                    return;
                }

                response.from.iconUrl = this.iconUrl;

                this.renderMessage(response);

                this.renderAttachments(response);

                this.renderSuggestedActions(response.suggestedActions);
            },

            renderMessage: function (message) {
                if (message.text || message.type == &quot;typing&quot;) {
                    this.chat.renderMessage(message, message.from);
                }
            },

            renderAttachments: function (data) {
                this.chat.renderAttachments(data, data.from);
            },

            renderSuggestedActions: function (suggestedActions) {
                var actions = [];

                if (suggestedActions &amp;&amp; suggestedActions.actions) {
                    actions = suggestedActions.actions;
                }

                this.chat.renderSuggestedActions(actions);
            }
        });

        var chat = $(&quot;#chat&quot;).kendoChat({
            post: function (args) {
                agent.postMessage(args);
            }
        }).data(&quot;kendoChat&quot;);

        var AdaptiveCardComponent = kendo.chat.Component.extend({
            init: function (options, view) {
                kendo.chat.Component.fn.init.call(this, options, view);

                var adaptiveCard = new AdaptiveCards.AdaptiveCard();

                adaptiveCard.hostConfig = new AdaptiveCards.HostConfig({
                    fontFamily: &quot;Segoe UI, Helvetica Neue, sans-serif&quot;
                });

                adaptiveCard.parse(options);

                var bodyElement = $(&quot;&lt;div>&quot;).addClass(&quot;k-card-body&quot;).append(adaptiveCard.render());
                this.element.addClass(&quot;k-card&quot;).append(bodyElement);
            }
        });

        kendo.chat.registerComponent(&quot;application/vnd.microsoft.card.adaptive&quot;, AdaptiveCardComponent);

    


                
    
        
    
        
            
                    
                        index.html
                    
            
        
            
    

    






    
        Also available for:
        
            
                    ASP.NET MVC
                    ASP.NET Core
                    JSP
                    PHP
            
        
        
            
                API REFERENCE
                
                    





                
            
    



                
                    &lt;div id=&quot;example&quot;>
    &lt;style>
        html .k-chat .k-card-deck .k-card {
            flex-basis: 230px;
        }

        .k-card > img.k-card-image {
            height: 115px;
            display: block;
        }
    &lt;/style>

    &lt;div class=&quot;demo-section&quot;>
        &lt;div id=&quot;chat&quot;>&lt;/div>
    &lt;/div>

    &lt;!-- Load Bot Framework Client API -->
    &lt;script src=&quot;https://unpkg.com/botframework-directlinejs@0.11.5/dist/directline.js&quot;>&lt;/script>

    &lt;!-- Load Adaptive Cards Client API -->
    &lt;script src=&quot;https://unpkg.com/adaptivecards@2.10.0/dist/adaptivecards.min.js&quot;>&lt;/script>

    &lt;script>
        $.when(
            $.getScript(&quot;https://unpkg.com/botframework-directlinejs@0.11.5/dist/directline.js&quot;),
            $.getScript(&quot;https://unpkg.com/adaptivecards@2.10.0/dist/adaptivecards.min.js&quot;)
        ).done(function () {
            window.agent = new DirectLineAgent(chat, &quot;Y_ly-If6haE.cwA.PQE.ZwOOsq4MlHcD3_YLFI-t9oW6L6DXMMBoi67LBz9WaWA&quot;);
        });

        window.DirectLineAgent = kendo.Class.extend({
            init: function (chat, secret) {
                this.chat = chat;
                this.iconUrl = &quot;../content/chat/VacationBot.png&quot;;

                this.agent = new DirectLine.DirectLine({ secret: secret });

                this.agent.activity$.subscribe($.proxy(this.onResponse, this));
            },

            postMessage: function (args) {
                var postArgs = {
                    text: args.text,
                    type: args.type,
                    timestamp: args.timestamp,
                    from: args.from
                };

                this.agent.postActivity(postArgs)
                    .subscribe();
            },

            onResponse: function (response) {
                if (response.from.id === this.chat.getUser().id) {
                    return;
                }

                response.from.iconUrl = this.iconUrl;

                this.renderMessage(response);

                this.renderAttachments(response);

                this.renderSuggestedActions(response.suggestedActions);
            },

            renderMessage: function (message) {
                if (message.text || message.type == &quot;typing&quot;) {
                    this.chat.renderMessage(message, message.from);
                }
            },

            renderAttachments: function (data) {
                this.chat.renderAttachments(data, data.from);
            },

            renderSuggestedActions: function (suggestedActions) {
                var actions = [];

                if (suggestedActions &amp;&amp; suggestedActions.actions) {
                    actions = suggestedActions.actions;
                }

                this.chat.renderSuggestedActions(actions);
            }
        });

        var chat = $(&quot;#chat&quot;).kendoChat({
            post: function (args) {
                agent.postMessage(args);
            }
        }).data(&quot;kendoChat&quot;);

        var AdaptiveCardComponent = kendo.chat.Component.extend({
            init: function (options, view) {
                kendo.chat.Component.fn.init.call(this, options, view);

                var adaptiveCard = new AdaptiveCards.AdaptiveCard();

                adaptiveCard.hostConfig = new AdaptiveCards.HostConfig({
                    fontFamily: &quot;Segoe UI, Helvetica Neue, sans-serif&quot;
                });

                adaptiveCard.parse(options);

                var bodyElement = $(&quot;&lt;div>&quot;).addClass(&quot;k-card-body&quot;).append(adaptiveCard.render());
                this.element.addClass(&quot;k-card&quot;).append(bodyElement);
            }
        });

        kendo.chat.registerComponent(&quot;application/vnd.microsoft.card.adaptive&quot;, AdaptiveCardComponent);

    &lt;/script>
&lt;/div>

                

                
                    
                        
                            
                                
                                    
                                        
                                    
                                    
                                        
                                            
                                                The Chat component is part of Kendo UI for jQuery, a professional grade UI library with 110+ components for building modern and feature-rich applications. To try it out sign up for a free 30-day trial.
                                            
                                        
                                    
                                
                                
                                    Download Free Trial
                                
                            
                        
                    

                
            

            
                Description
                
                    The Kendo UI Chat widget allows for integration with any bot framework, due to its simplicity, flexible API and customizable templates.This demo demonstrates how the widget can be connected to a chat bot created in Microsoft&quot; , &quot;'&quot; , &quot;s Bot Framework, using the DirectLineJS client library. The sample shows off out of the box features of the Chat widget, such as the hero cards and suggested actions, as well as the ability to define custom components, which let you use javascript to render any content. In this particular case we use the AdaptiveCards client API to render the adaptive cards returned by the service.
                
            
            
            
                Support &amp; Learning Resources
                
                        jQuery Chat Documentation Overview
                        jQuery Chat API
                        jQuery Chat Forums
                        Kendo UI for jQuery Chat Product Homepage
                    Kendo UI for jQuery Knowledge Base
                
            

                
                    Additional Resources
                    
                        Kendo UI for jQuery Product Overview
                        Kendo UI for jQuery Blog
                        Kendo UI for jQuery Videos
                        Kendo UI for jQuery Roadmap
                        Kendo UI for jQuery Pricing
                        Kendo UI for jQuery Training
                    
                
        
    &quot;) or . = concat(&quot;
                

                
                    
                        
                            Components
                            / 
                        
                        
                            Chat
                            / 
                        
                        
                            Travel
                        
                    
                

                

                
                
jQuery
Chat
Travel                

                
                


    
        
            .a{fill:#a2d7f1;}
        
        Run Live Demo
    

    
        
            
                .a{fill:#a2d7f1;}
            
            Loading Demo...
        
    

    
        
            
                EXAMPLE
            
            
                VIEW SOURCE
            
            
            
                
                    
                        Edit in Kendo UI Dojo
                    
                    
                        





                    
                
            
            
            

    window.selectedTheme = &quot;default-ocean-blue&quot;;
    window.selectedThemeCommon = &quot;common-empty&quot;;


Change Theme

    
            

            
            
            


            
                default
            
        
        
            






        
    
    
        
                    
                        
                            Default Theme
                        
                        
                                
                                    

            
            
            


                                    Main
                                
                                
                                    

            
            
            


                                    Main Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Nordic
                                
                                
                                    

            
            
            


                                    Ocean Blue
                                
                                
                                    

            
            
            


                                    Ocean Blue A11Y
                                        
                                            
  accessibility
  


                                        
                                
                                
                                    

            
            
            


                                    Purple
                                
                                
                                    

            
            
            


                                    Turquoise
                                
                        
                    
                    
                        
                            Bootstrap Theme
                        
                        
                                
                                    

            
            
            


                                    Main
                                
                                
                                    

            
            
            


                                    Bootstrap 3
                                
                                
                                    

            
            
            


                                    Bootstrap 3 Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Main Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Nordic
                                
                                
                                    

            
            
            


                                    Turquoise
                                
                                
                                    

            
            
            


                                    Turquoise Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Urban
                                
                                
                                    

            
            
            


                                    Vintage
                                
                        
                    
                    
                        
                            Material Theme
                        
                        
                                
                                    

            
            
            


                                    Main
                                
                                
                                    

            
            
            


                                    Arctic
                                
                                
                                    

            
            
            


                                    Lime Dark
                                
                                
                                    

            
            
            


                                    Main Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Nova
                                
                        
                    
                    
                        
                            Classic Theme
                        
                        
                                
                                    

            
            
            


                                    Main
                                
                                
                                    

            
            
            


                                    Green
                                
                                
                                    

            
            
            


                                    Green Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Lavender
                                
                                
                                    

            
            
            


                                    Lavender Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Main Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Metro
                                
                                
                                    

            
            
            


                                    Metro Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Moonlight
                                
                                
                                    

            
            
            


                                    Opal
                                
                                
                                    

            
            
            


                                    Opal Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Silver
                                
                                
                                    

            
            
            


                                    Silver Dark
                                        
                                            



                                        
                                
                                
                                    

            
            
            


                                    Uniform
                                
                        
                    
                    
                        
                            Fluent Theme
                        
                        
                                
                                    

            
            
            


                                    Main
                                
                        
                    
        
        
            
                
                    



	
		
	
































                
                Launch Theme Builder
            
        
    

        
    
    
                    
    
        html .k-chat .k-card-deck .k-card {
            flex-basis: 230px;
        }

        .k-card > img.k-card-image {
            height: 115px;
            display: block;
        }
    

    
        Type a message...
    

    
    

    
    

    
        $.when(
            $.getScript(&quot;https://unpkg.com/botframework-directlinejs@0.11.5/dist/directline.js&quot;),
            $.getScript(&quot;https://unpkg.com/adaptivecards@2.10.0/dist/adaptivecards.min.js&quot;)
        ).done(function () {
            window.agent = new DirectLineAgent(chat, &quot;Y_ly-If6haE.cwA.PQE.ZwOOsq4MlHcD3_YLFI-t9oW6L6DXMMBoi67LBz9WaWA&quot;);
        });

        window.DirectLineAgent = kendo.Class.extend({
            init: function (chat, secret) {
                this.chat = chat;
                this.iconUrl = &quot;../content/chat/VacationBot.png&quot;;

                this.agent = new DirectLine.DirectLine({ secret: secret });

                this.agent.activity$.subscribe($.proxy(this.onResponse, this));
            },

            postMessage: function (args) {
                var postArgs = {
                    text: args.text,
                    type: args.type,
                    timestamp: args.timestamp,
                    from: args.from
                };

                this.agent.postActivity(postArgs)
                    .subscribe();
            },

            onResponse: function (response) {
                if (response.from.id === this.chat.getUser().id) {
                    return;
                }

                response.from.iconUrl = this.iconUrl;

                this.renderMessage(response);

                this.renderAttachments(response);

                this.renderSuggestedActions(response.suggestedActions);
            },

            renderMessage: function (message) {
                if (message.text || message.type == &quot;typing&quot;) {
                    this.chat.renderMessage(message, message.from);
                }
            },

            renderAttachments: function (data) {
                this.chat.renderAttachments(data, data.from);
            },

            renderSuggestedActions: function (suggestedActions) {
                var actions = [];

                if (suggestedActions &amp;&amp; suggestedActions.actions) {
                    actions = suggestedActions.actions;
                }

                this.chat.renderSuggestedActions(actions);
            }
        });

        var chat = $(&quot;#chat&quot;).kendoChat({
            post: function (args) {
                agent.postMessage(args);
            }
        }).data(&quot;kendoChat&quot;);

        var AdaptiveCardComponent = kendo.chat.Component.extend({
            init: function (options, view) {
                kendo.chat.Component.fn.init.call(this, options, view);

                var adaptiveCard = new AdaptiveCards.AdaptiveCard();

                adaptiveCard.hostConfig = new AdaptiveCards.HostConfig({
                    fontFamily: &quot;Segoe UI, Helvetica Neue, sans-serif&quot;
                });

                adaptiveCard.parse(options);

                var bodyElement = $(&quot;&lt;div>&quot;).addClass(&quot;k-card-body&quot;).append(adaptiveCard.render());
                this.element.addClass(&quot;k-card&quot;).append(bodyElement);
            }
        });

        kendo.chat.registerComponent(&quot;application/vnd.microsoft.card.adaptive&quot;, AdaptiveCardComponent);

    


                
    
        
    
        
            
                    
                        index.html
                    
            
        
            
    

    






    
        Also available for:
        
            
                    ASP.NET MVC
                    ASP.NET Core
                    JSP
                    PHP
            
        
        
            
                API REFERENCE
                
                    





                
            
    



                
                    &lt;div id=&quot;example&quot;>
    &lt;style>
        html .k-chat .k-card-deck .k-card {
            flex-basis: 230px;
        }

        .k-card > img.k-card-image {
            height: 115px;
            display: block;
        }
    &lt;/style>

    &lt;div class=&quot;demo-section&quot;>
        &lt;div id=&quot;chat&quot;>&lt;/div>
    &lt;/div>

    &lt;!-- Load Bot Framework Client API -->
    &lt;script src=&quot;https://unpkg.com/botframework-directlinejs@0.11.5/dist/directline.js&quot;>&lt;/script>

    &lt;!-- Load Adaptive Cards Client API -->
    &lt;script src=&quot;https://unpkg.com/adaptivecards@2.10.0/dist/adaptivecards.min.js&quot;>&lt;/script>

    &lt;script>
        $.when(
            $.getScript(&quot;https://unpkg.com/botframework-directlinejs@0.11.5/dist/directline.js&quot;),
            $.getScript(&quot;https://unpkg.com/adaptivecards@2.10.0/dist/adaptivecards.min.js&quot;)
        ).done(function () {
            window.agent = new DirectLineAgent(chat, &quot;Y_ly-If6haE.cwA.PQE.ZwOOsq4MlHcD3_YLFI-t9oW6L6DXMMBoi67LBz9WaWA&quot;);
        });

        window.DirectLineAgent = kendo.Class.extend({
            init: function (chat, secret) {
                this.chat = chat;
                this.iconUrl = &quot;../content/chat/VacationBot.png&quot;;

                this.agent = new DirectLine.DirectLine({ secret: secret });

                this.agent.activity$.subscribe($.proxy(this.onResponse, this));
            },

            postMessage: function (args) {
                var postArgs = {
                    text: args.text,
                    type: args.type,
                    timestamp: args.timestamp,
                    from: args.from
                };

                this.agent.postActivity(postArgs)
                    .subscribe();
            },

            onResponse: function (response) {
                if (response.from.id === this.chat.getUser().id) {
                    return;
                }

                response.from.iconUrl = this.iconUrl;

                this.renderMessage(response);

                this.renderAttachments(response);

                this.renderSuggestedActions(response.suggestedActions);
            },

            renderMessage: function (message) {
                if (message.text || message.type == &quot;typing&quot;) {
                    this.chat.renderMessage(message, message.from);
                }
            },

            renderAttachments: function (data) {
                this.chat.renderAttachments(data, data.from);
            },

            renderSuggestedActions: function (suggestedActions) {
                var actions = [];

                if (suggestedActions &amp;&amp; suggestedActions.actions) {
                    actions = suggestedActions.actions;
                }

                this.chat.renderSuggestedActions(actions);
            }
        });

        var chat = $(&quot;#chat&quot;).kendoChat({
            post: function (args) {
                agent.postMessage(args);
            }
        }).data(&quot;kendoChat&quot;);

        var AdaptiveCardComponent = kendo.chat.Component.extend({
            init: function (options, view) {
                kendo.chat.Component.fn.init.call(this, options, view);

                var adaptiveCard = new AdaptiveCards.AdaptiveCard();

                adaptiveCard.hostConfig = new AdaptiveCards.HostConfig({
                    fontFamily: &quot;Segoe UI, Helvetica Neue, sans-serif&quot;
                });

                adaptiveCard.parse(options);

                var bodyElement = $(&quot;&lt;div>&quot;).addClass(&quot;k-card-body&quot;).append(adaptiveCard.render());
                this.element.addClass(&quot;k-card&quot;).append(bodyElement);
            }
        });

        kendo.chat.registerComponent(&quot;application/vnd.microsoft.card.adaptive&quot;, AdaptiveCardComponent);

    &lt;/script>
&lt;/div>

                

                
                    
                        
                            
                                
                                    
                                        
                                    
                                    
                                        
                                            
                                                The Chat component is part of Kendo UI for jQuery, a professional grade UI library with 110+ components for building modern and feature-rich applications. To try it out sign up for a free 30-day trial.
                                            
                                        
                                    
                                
                                
                                    Download Free Trial
                                
                            
                        
                    

                
            

            
                Description
                
                    The Kendo UI Chat widget allows for integration with any bot framework, due to its simplicity, flexible API and customizable templates.This demo demonstrates how the widget can be connected to a chat bot created in Microsoft&quot; , &quot;'&quot; , &quot;s Bot Framework, using the DirectLineJS client library. The sample shows off out of the box features of the Chat widget, such as the hero cards and suggested actions, as well as the ability to define custom components, which let you use javascript to render any content. In this particular case we use the AdaptiveCards client API to render the adaptive cards returned by the service.
                
            
            
            
                Support &amp; Learning Resources
                
                        jQuery Chat Documentation Overview
                        jQuery Chat API
                        jQuery Chat Forums
                        Kendo UI for jQuery Chat Product Homepage
                    Kendo UI for jQuery Knowledge Base
                
            

                
                    Additional Resources
                    
                        Kendo UI for jQuery Product Overview
                        Kendo UI for jQuery Blog
                        Kendo UI for jQuery Videos
                        Kendo UI for jQuery Roadmap
                        Kendo UI for jQuery Pricing
                        Kendo UI for jQuery Training
                    
                
        
    &quot;))]</value>
      <webElementGuid>1a5489d7-05d7-4eeb-8980-aede03cc72fb</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
