/*
 * Decompiled with CFR 0.152.
 */
package org.openapitools.codegen.languages;

import com.google.common.collect.ImmutableMap;
import com.samskivert.mustache.Mustache;
import com.samskivert.mustache.Template;
import io.swagger.v3.oas.models.media.Discriminator;
import io.swagger.v3.oas.models.media.Schema;
import java.io.File;
import java.io.IOException;
import java.io.Writer;
import java.math.BigDecimal;
import java.math.BigInteger;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.EnumSet;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;
import java.util.stream.Collectors;
import joptsimple.internal.Strings;
import org.openapitools.codegen.CliOption;
import org.openapitools.codegen.CodegenConfig;
import org.openapitools.codegen.CodegenDiscriminator;
import org.openapitools.codegen.CodegenModel;
import org.openapitools.codegen.CodegenOperation;
import org.openapitools.codegen.CodegenParameter;
import org.openapitools.codegen.CodegenProperty;
import org.openapitools.codegen.CodegenType;
import org.openapitools.codegen.SupportingFile;
import org.openapitools.codegen.languages.AbstractRustCodegen;
import org.openapitools.codegen.meta.features.ClientModificationFeature;
import org.openapitools.codegen.meta.features.DocumentationFeature;
import org.openapitools.codegen.meta.features.GlobalFeature;
import org.openapitools.codegen.meta.features.ParameterFeature;
import org.openapitools.codegen.meta.features.SchemaSupportFeature;
import org.openapitools.codegen.meta.features.SecurityFeature;
import org.openapitools.codegen.meta.features.WireFormatFeature;
import org.openapitools.codegen.model.ModelMap;
import org.openapitools.codegen.model.ModelsMap;
import org.openapitools.codegen.model.OperationMap;
import org.openapitools.codegen.model.OperationsMap;
import org.openapitools.codegen.templating.mustache.ReplaceAllLambda;
import org.openapitools.codegen.utils.ModelUtils;
import org.openapitools.codegen.utils.StringUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class RustClientCodegen
extends AbstractRustCodegen
implements CodegenConfig {
    private final Logger LOGGER = LoggerFactory.getLogger(RustClientCodegen.class);
    private boolean useSingleRequestParameter = false;
    private boolean supportAsync = true;
    private boolean supportMiddleware = false;
    private boolean supportTokenSource = false;
    private boolean supportMultipleResponses = false;
    private boolean withAWSV4Signature = false;
    private boolean preferUnsignedInt = false;
    private boolean bestFitInt = false;
    private boolean avoidBoxedModels = false;
    public static final String PACKAGE_NAME = "packageName";
    public static final String EXTERN_CRATE_NAME = "externCrateName";
    public static final String PACKAGE_VERSION = "packageVersion";
    public static final String HYPER_LIBRARY = "hyper";
    public static final String HYPER0X_LIBRARY = "hyper0x";
    public static final String REQWEST_LIBRARY = "reqwest";
    public static final String REQWEST_TRAIT_LIBRARY = "reqwest-trait";
    public static final String REQWEST_TRAIT_LIBRARY_ATTR = "reqwestTrait";
    public static final String SUPPORT_ASYNC = "supportAsync";
    public static final String SUPPORT_MIDDLEWARE = "supportMiddleware";
    public static final String SUPPORT_TOKEN_SOURCE = "supportTokenSource";
    public static final String SUPPORT_MULTIPLE_RESPONSES = "supportMultipleResponses";
    public static final String PREFER_UNSIGNED_INT = "preferUnsignedInt";
    public static final String BEST_FIT_INT = "bestFitInt";
    public static final String AVOID_BOXED_MODELS = "avoidBoxedModels";
    public static final String TOP_LEVEL_API_CLIENT = "topLevelApiClient";
    public static final String MOCKALL = "mockall";
    public static final String BON_BUILDER = "useBonBuilder";
    protected String packageName = "openapi";
    protected String packageVersion = "1.0.0";
    protected String apiDocPath = "docs/";
    protected String modelDocPath = "docs/";
    protected String apiFolder = "src/apis";
    protected String modelFolder = "src/models";
    private boolean hasUUIDs = false;

    @Override
    public CodegenType getTag() {
        return CodegenType.CLIENT;
    }

    @Override
    public String getName() {
        return "rust";
    }

    @Override
    public String getHelp() {
        return "Generates a Rust client library (beta).";
    }

    public RustClientCodegen() {
        this.modifyFeatureSet(features -> features.includeDocumentationFeatures(DocumentationFeature.Readme).wireFormatFeatures(EnumSet.of(WireFormatFeature.JSON, WireFormatFeature.XML, WireFormatFeature.Custom)).securityFeatures(EnumSet.of(SecurityFeature.BasicAuth, SecurityFeature.BearerToken, SecurityFeature.ApiKey, SecurityFeature.OAuth2_Implicit, SecurityFeature.AWSV4Signature)).excludeGlobalFeatures(GlobalFeature.XMLStructureDefinitions, GlobalFeature.Callbacks, GlobalFeature.LinkObjects, GlobalFeature.ParameterStyling).excludeSchemaSupportFeatures(SchemaSupportFeature.Polymorphism).excludeParameterFeatures(ParameterFeature.Cookie).includeSchemaSupportFeatures(SchemaSupportFeature.oneOf).includeClientModificationFeatures(ClientModificationFeature.BasePath, ClientModificationFeature.UserAgent));
        this.outputFolder = "generated-code/rust";
        this.modelTemplateFiles.put("model.mustache", ".rs");
        this.modelDocTemplateFiles.put("model_doc.mustache", ".md");
        this.apiDocTemplateFiles.put("api_doc.mustache", ".md");
        this.hideGenerationTimestamp = Boolean.TRUE;
        this.templateDir = "rust";
        this.embeddedTemplateDir = "rust";
        this.defaultIncludes = new HashSet<String>(Arrays.asList("map", "array"));
        this.languageSpecificPrimitives = new HashSet<String>(Arrays.asList("i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "isize", "usize", "char", "bool", "str", "String"));
        this.instantiationTypes.clear();
        this.typeMapping.clear();
        this.typeMapping.put("integer", "i32");
        this.typeMapping.put("long", "i64");
        this.typeMapping.put("number", "f64");
        this.typeMapping.put("float", "f32");
        this.typeMapping.put("double", "f64");
        this.typeMapping.put("boolean", "bool");
        this.typeMapping.put("string", "String");
        this.typeMapping.put("array", "Vec");
        this.typeMapping.put("map", "std::collections::HashMap");
        this.typeMapping.put("UUID", "uuid::Uuid");
        this.typeMapping.put("URI", "String");
        this.typeMapping.put("date", "string");
        this.typeMapping.put("DateTime", "String");
        this.typeMapping.put("password", "String");
        this.typeMapping.put("decimal", "String");
        this.typeMapping.put("file", "std::path::PathBuf");
        this.typeMapping.put("binary", "crate::models::File");
        this.typeMapping.put("ByteArray", "String");
        this.typeMapping.put("object", "serde_json::Value");
        this.typeMapping.put("AnyType", "serde_json::Value");
        this.cliOptions.clear();
        this.cliOptions.add(new CliOption(PACKAGE_NAME, "Rust package name (convention: lowercase).").defaultValue("openapi"));
        this.cliOptions.add(new CliOption(PACKAGE_VERSION, "Rust package version.").defaultValue("1.0.0"));
        this.cliOptions.add(new CliOption("hideGenerationTimestamp", "Hides the generation timestamp when files are generated.").defaultValue(Boolean.TRUE.toString()));
        this.cliOptions.add(new CliOption("useSingleRequestParameter", "Setting this property to true will generate functions with a single argument containing all API endpoint parameters instead of one argument per parameter.", "boolean").defaultValue(Boolean.FALSE.toString()));
        this.cliOptions.add(new CliOption(SUPPORT_ASYNC, "If set, generate async function call instead. This option is for 'reqwest' library only", "boolean").defaultValue(Boolean.TRUE.toString()));
        this.cliOptions.add(new CliOption(SUPPORT_MIDDLEWARE, "If set, add support for reqwest-middleware. This option is for 'reqwest' and 'reqwest-trait' library only", "boolean").defaultValue(Boolean.FALSE.toString()));
        this.cliOptions.add(new CliOption(SUPPORT_TOKEN_SOURCE, "If set, add support for google-cloud-token. This option is for 'reqwest' and 'reqwest-trait' library only and requires the 'supportAsync' option", "boolean").defaultValue(Boolean.FALSE.toString()));
        this.cliOptions.add(new CliOption(SUPPORT_MULTIPLE_RESPONSES, "If set, return type wraps an enum of all possible 2xx schemas. This option is for 'reqwest' and 'reqwest-trait' library only", "boolean").defaultValue(Boolean.FALSE.toString()));
        this.cliOptions.add(new CliOption("enumNameSuffix", "Suffix that will be appended to all enum names.").defaultValue(this.enumSuffix));
        this.cliOptions.add(new CliOption("withAWSV4Signature", "whether to include AWS v4 signature support", "boolean").defaultValue(Boolean.FALSE.toString()));
        this.cliOptions.add(new CliOption(PREFER_UNSIGNED_INT, "Prefer unsigned integers where minimum value is >= 0", "boolean").defaultValue(Boolean.FALSE.toString()));
        this.cliOptions.add(new CliOption(BEST_FIT_INT, "Use best fitting integer type where minimum or maximum is set", "boolean").defaultValue(Boolean.FALSE.toString()));
        this.cliOptions.add(new CliOption(AVOID_BOXED_MODELS, "If set, `Box<T>` will not be used for models", "boolean").defaultValue(Boolean.FALSE.toString()));
        this.cliOptions.add(new CliOption(MOCKALL, "Adds `#[automock]` from the mockall crate to api traits. This option is for 'reqwest-trait' library only", "boolean").defaultValue(Boolean.FALSE.toString()));
        this.cliOptions.add(new CliOption(TOP_LEVEL_API_CLIENT, "Creates a top level `Api` trait and `ApiClient` struct that contain all Apis. This option is for 'reqwest-trait' library only", "boolean").defaultValue(Boolean.FALSE.toString()));
        this.cliOptions.add(new CliOption(BON_BUILDER, "Use the bon crate for building parameter types. This option is for the 'reqwest-trait' library only", "boolean").defaultValue(Boolean.FALSE.toString()));
        this.supportedLibraries.put(HYPER_LIBRARY, "HTTP client: Hyper (v1.x).");
        this.supportedLibraries.put(HYPER0X_LIBRARY, "HTTP client: Hyper (v0.x).");
        this.supportedLibraries.put(REQWEST_LIBRARY, "HTTP client: Reqwest.");
        this.supportedLibraries.put(REQWEST_TRAIT_LIBRARY, "HTTP client: Reqwest (trait based).");
        CliOption libraryOption = new CliOption("library", "library template (sub-template) to use.");
        libraryOption.setEnum(this.supportedLibraries);
        libraryOption.setDefault(REQWEST_LIBRARY);
        this.cliOptions.add(libraryOption);
        this.setLibrary(REQWEST_LIBRARY);
    }

    @Override
    public CodegenModel fromModel(String name, Schema model) {
        CodegenModel mdl = super.fromModel(name, model);
        if (mdl.getComposedSchemas() != null && mdl.getComposedSchemas().getOneOf() != null && !mdl.getComposedSchemas().getOneOf().isEmpty()) {
            List<CodegenProperty> newOneOfs = mdl.getComposedSchemas().getOneOf().stream().map(CodegenProperty::clone).collect(Collectors.toList());
            List<Schema> schemas = ModelUtils.getInterfaces(model);
            if (newOneOfs.size() != schemas.size()) {
                throw new RuntimeException("oneOf size does not match the model");
            }
            Map<String, String> refsMapping = Optional.ofNullable(model.getDiscriminator()).map(Discriminator::getMapping).orElse(Collections.<String,String>emptyMap());
            Map<String, List<String>> mappedNamesByRef = refsMapping.entrySet().stream().collect(Collectors.groupingBy(Map.Entry::getValue, Collectors.mapping(Map.Entry::getKey, Collectors.toList())));
            for (int i = 0; i < newOneOfs.size(); ++i) {
                CodegenProperty oneOf = (CodegenProperty)newOneOfs.get(i);
                Schema schema = schemas.get(i);
                if (mappedNamesByRef.containsKey(schema.get$ref())) {
                    List<String> names = mappedNamesByRef.get(schema.get$ref());
                    String mappedName = (String)names.remove(0);
                    oneOf.setBaseName(mappedName);
                    oneOf.setName(this.toModelName(mappedName));
                    continue;
                }
                if (!org.apache.commons.lang3.StringUtils.isEmpty(schema.get$ref())) {
                    String refName = ModelUtils.getSimpleRef(schema.get$ref());
                    if (refName == null) continue;
                    String modelName = this.toModelName(refName);
                    oneOf.setName(modelName);
                    oneOf.setBaseName(refName);
                    continue;
                }
                String oneOfName = Optional.ofNullable(schema.getTitle()).orElseGet(schema::getType);
                oneOf.setName(this.toModelName(oneOfName));
            }
            mdl.getComposedSchemas().setOneOf(newOneOfs);
        }
        return mdl;
    }

    @Override
    public ModelsMap postProcessModels(ModelsMap objs) {
        block0: for (ModelMap model : objs.getModels()) {
            CodegenModel cm = model.getModel();
            if (cm.discriminator != null) {
                String reserved_var_name = cm.discriminator.getPropertyBaseName();
                for (CodegenProperty cp : cm.vars) {
                    if (!cp.baseName.equals(reserved_var_name)) continue;
                    cm.vars.remove(cp);
                    break;
                }
                if (cm.discriminator.getMapping() != null && RustClientCodegen.hasDuplicateValues(cm.discriminator.getMapping())) {
                    Map<String, List<String>> inverted = RustClientCodegen.invertMap(cm.discriminator.getMapping());
                    for (Map.Entry<String, List<String>> s2 : inverted.entrySet()) {
                        if (s2.getValue().size() <= 1) continue;
                        this.LOGGER.debug("Found duplicated enum model (" + s2.getKey() + ") in model " + cm.name + ". Adding suffix to model names.");
                        for (CodegenDiscriminator.MappedModel m4 : cm.discriminator.getMappedModels()) {
                            if (!s2.getValue().contains(m4.getMappingName())) continue;
                            m4.setModelName(m4.getModelName() + StringUtils.camelize(m4.getMappingName()));
                        }
                    }
                }
            }
            for (CodegenProperty cp : cm.vars) {
                if (!cp.isByteArray) continue;
                cm.vendorExtensions.put("x-rust-has-byte-array", true);
                continue block0;
            }
        }
        // Emulate the committed hand-edit on PoamResponseGetSystems.data: the
        // eMASS spec returns `data: [null]` for empty lists, so each array
        // element is allowed to be null (Option<...> inner) to mirror emass's
        // loose reference tool.
        for (ModelMap model : objs.getModels()) {
            CodegenModel cm = model.getModel();
            if (!"PoamResponseGetSystems".equals(cm.classname)) continue;
            for (CodegenProperty cp : cm.vars) {
                if (!"data".equals(cp.name)) continue;
                cp.dataType = "Vec<Option<models::PoamResponseGetSystemsDataInner>>";
                cp.vendorExtensions.put("x-d3-comment",
                    "    // eMASS returns `data: [null]` for empty lists. To mirror emass's loose\n" +
                    "    // reference tool's loose deserialization, each array element is allowed\n" +
                    "    // to be null; the POC's output pipeline compacts null elements away\n" +
                    "    // (as _to_hash does).");
            }
        }
        return this.postProcessModelsEnum(objs);
    }

    @Override
    public void processOpts() {
        super.processOpts();
        if (this.additionalProperties.containsKey("withAWSV4Signature")) {
            this.withAWSV4Signature = Boolean.parseBoolean(this.additionalProperties.get("withAWSV4Signature").toString());
        }
        if (this.additionalProperties.containsKey("enumNameSuffix")) {
            this.enumSuffix = this.additionalProperties.get("enumNameSuffix").toString();
        }
        if (this.additionalProperties.containsKey(PACKAGE_NAME)) {
            this.setPackageName((String)this.additionalProperties.get(PACKAGE_NAME));
        } else {
            this.setPackageName("openapi");
        }
        if (this.additionalProperties.containsKey(PACKAGE_VERSION)) {
            this.setPackageVersion((String)this.additionalProperties.get(PACKAGE_VERSION));
        } else if (this.openAPI != null && this.openAPI.getInfo() != null && this.openAPI.getInfo().getVersion() != null) {
            this.setPackageVersion(this.openAPI.getInfo().getVersion());
        }
        if (this.additionalProperties.containsKey("useSingleRequestParameter")) {
            this.setUseSingleRequestParameter(this.convertPropertyToBoolean("useSingleRequestParameter"));
        }
        this.writePropertyBack("useSingleRequestParameter", this.getUseSingleRequestParameter());
        if (this.additionalProperties.containsKey(SUPPORT_ASYNC)) {
            this.setSupportAsync(this.convertPropertyToBoolean(SUPPORT_ASYNC));
        }
        this.writePropertyBack(SUPPORT_ASYNC, this.getSupportAsync());
        if (this.additionalProperties.containsKey(SUPPORT_MIDDLEWARE)) {
            this.setSupportMiddleware(this.convertPropertyToBoolean(SUPPORT_MIDDLEWARE));
        }
        this.writePropertyBack(SUPPORT_MIDDLEWARE, this.getSupportMiddleware());
        if (this.additionalProperties.containsKey(SUPPORT_TOKEN_SOURCE)) {
            this.setSupportTokenSource(this.convertPropertyToBoolean(SUPPORT_TOKEN_SOURCE));
        }
        this.writePropertyBack(SUPPORT_TOKEN_SOURCE, this.getSupportTokenSource());
        if (this.additionalProperties.containsKey(SUPPORT_MULTIPLE_RESPONSES)) {
            this.setSupportMultipleReturns(this.convertPropertyToBoolean(SUPPORT_MULTIPLE_RESPONSES));
        }
        this.writePropertyBack(SUPPORT_MULTIPLE_RESPONSES, this.getSupportMultipleReturns());
        if (this.additionalProperties.containsKey(PREFER_UNSIGNED_INT)) {
            this.setPreferUnsignedInt(this.convertPropertyToBoolean(PREFER_UNSIGNED_INT));
        }
        this.writePropertyBack(PREFER_UNSIGNED_INT, this.getPreferUnsignedInt());
        if (this.additionalProperties.containsKey(BEST_FIT_INT)) {
            this.setBestFitInt(this.convertPropertyToBoolean(BEST_FIT_INT));
        }
        this.writePropertyBack(BEST_FIT_INT, this.getBestFitInt());
        if (this.additionalProperties.containsKey(AVOID_BOXED_MODELS)) {
            this.setAvoidBoxedModels(this.convertPropertyToBoolean(AVOID_BOXED_MODELS));
        }
        this.writePropertyBack(AVOID_BOXED_MODELS, this.getAvoidBoxedModels());
        this.additionalProperties.put(PACKAGE_NAME, this.packageName);
        this.additionalProperties.put(PACKAGE_VERSION, this.packageVersion);
        this.additionalProperties.put(EXTERN_CRATE_NAME, this.getExternCrateName());
        this.additionalProperties.put("apiDocPath", this.apiDocPath);
        this.additionalProperties.put("modelDocPath", this.modelDocPath);
        if (HYPER_LIBRARY.equals(this.getLibrary())) {
            this.additionalProperties.put(HYPER_LIBRARY, "true");
        } else if (HYPER0X_LIBRARY.equals(this.getLibrary())) {
            this.additionalProperties.put(HYPER_LIBRARY, "true");
            this.additionalProperties.put(HYPER0X_LIBRARY, "true");
        } else if (REQWEST_LIBRARY.equals(this.getLibrary())) {
            this.additionalProperties.put(REQWEST_LIBRARY, "true");
        } else if (REQWEST_TRAIT_LIBRARY.equals(this.getLibrary())) {
            this.additionalProperties.put(REQWEST_TRAIT_LIBRARY_ATTR, "true");
        } else {
            this.LOGGER.error("Unknown library option (-l/--library): {}", (Object)this.getLibrary());
        }
        this.apiTemplateFiles.put(this.getLibrary() + "/api.mustache", ".rs");
        this.modelPackage = this.packageName;
        this.apiPackage = this.packageName;
        this.supportingFiles.add(new SupportingFile("README.mustache", "", "README.md"));
        this.supportingFiles.add(new SupportingFile("git_push.sh.mustache", "", "git_push.sh"));
        this.supportingFiles.add(new SupportingFile("gitignore.mustache", "", ".gitignore"));
        this.supportingFiles.add(new SupportingFile(".travis.yml", "", ".travis.yml"));
        this.supportingFiles.add(new SupportingFile("model_mod.mustache", this.modelFolder, "mod.rs"));
        this.supportingFiles.add(new SupportingFile("lib.mustache", "src", "lib.rs"));
        this.supportingFiles.add(new SupportingFile("Cargo.mustache", "", "Cargo.toml"));
        this.supportingFiles.add(new SupportingFile(this.getLibrary() + "/api_mod.mustache", this.apiFolder, "mod.rs"));
        this.supportingFiles.add(new SupportingFile(this.getLibrary() + "/configuration.mustache", this.apiFolder, "configuration.rs"));
        if (HYPER_LIBRARY.equals(this.getLibrary())) {
            this.supportingFiles.add(new SupportingFile("request.rs", this.apiFolder, "request.rs"));
            this.supportingFiles.add(new SupportingFile(this.getLibrary() + "/client.mustache", this.apiFolder, "client.rs"));
        }
        this.additionalProperties.put("lambdaVersion", new Mustache.Lambda(){

            @Override
            public void execute(Template.Fragment fragment, Writer writer) throws IOException {
                Object content = fragment.execute();
                content = ((String)content).trim().replace("v", "");
                String[] contents = ((String)(content = ((String)content).replace("V", ""))).split("[.]");
                if (contents.length == 1) {
                    content = (String)content + ".0.0";
                } else if (contents.length == 2) {
                    content = (String)content + ".0";
                }
                writer.write((String)content);
            }
        });
    }

    private String getExternCrateName() {
        return this.packageName.replace('-', '_');
    }

    private boolean getSupportAsync() {
        return this.supportAsync;
    }

    private boolean getSupportMiddleware() {
        return this.supportMiddleware;
    }

    private boolean getSupportTokenSource() {
        return this.supportTokenSource;
    }

    public boolean getSupportMultipleReturns() {
        return this.supportMultipleResponses;
    }

    public void setSupportMultipleReturns(boolean supportMultipleResponses) {
        this.supportMultipleResponses = supportMultipleResponses;
    }

    public boolean getPreferUnsignedInt() {
        return this.preferUnsignedInt;
    }

    public boolean getBestFitInt() {
        return this.bestFitInt;
    }

    private boolean getUseSingleRequestParameter() {
        return this.useSingleRequestParameter;
    }

    public boolean getAvoidBoxedModels() {
        return this.avoidBoxedModels;
    }

    @Override
    public String apiFileFolder() {
        return (this.outputFolder + File.separator + this.apiFolder).replace("/", File.separator);
    }

    @Override
    public String modelFileFolder() {
        return (this.outputFolder + File.separator + this.modelFolder).replace("/", File.separator);
    }

    @Override
    public String apiDocFileFolder() {
        return (this.outputFolder + "/" + this.apiDocPath).replace('/', File.separatorChar);
    }

    @Override
    public String modelDocFileFolder() {
        return (this.outputFolder + "/" + this.modelDocPath).replace('/', File.separatorChar);
    }

    @Override
    public String getTypeDeclaration(Schema p) {
        Schema unaliasSchema = this.unaliasSchema(p);
        return super.getTypeDeclaration(unaliasSchema);
    }

    @Override
    public String getSchemaType(Schema p) {
        String schemaType = super.getSchemaType(p);
        String type = this.typeMapping.getOrDefault(schemaType, schemaType);
        if (Objects.equals(p.getType(), "integer")) {
            boolean unsigned;
            boolean bestFit = this.convertPropertyToBoolean(BEST_FIT_INT);
            boolean preferUnsigned = this.convertPropertyToBoolean(PREFER_UNSIGNED_INT);
            BigInteger minimum = Optional.ofNullable(p.getMinimum()).map(BigDecimal::toBigInteger).orElse(null);
            boolean exclusiveMinimum = Optional.ofNullable(p.getExclusiveMinimum()).orElse(false);
            boolean bl = unsigned = preferUnsigned && this.canFitIntoUnsigned(minimum, exclusiveMinimum);
            if (Strings.isNullOrEmpty(p.getFormat())) {
                if (bestFit) {
                    return this.bestFittingIntegerType(minimum, exclusiveMinimum, Optional.ofNullable(p.getMaximum()).map(BigDecimal::toBigInteger).orElse(null), Optional.ofNullable(p.getExclusiveMaximum()).orElse(false), preferUnsigned);
                }
                return unsigned ? "u32" : "i32";
            }
            switch (p.getFormat()) {
                case "int32": {
                    return unsigned ? "u32" : "i32";
                }
                case "int64": {
                    return unsigned ? "u64" : "i64";
                }
            }
        }
        return type;
    }

    @Override
    public void postProcessModelProperty(CodegenModel model, CodegenProperty property) {
        super.postProcessModelProperty(model, property);
        if (property.isNullable && !property.required) {
            this.additionalProperties.put("serdeWith", true);
        }
        if (property.isByteArray) {
            this.additionalProperties.put("serdeWith", true);
        }
    }

    @Override
    public void postProcessParameter(CodegenParameter parameter) {
        super.postProcessParameter(parameter);
        Object inFunctionIdentifier = "p_" + parameterScope(parameter) + "_" + parameterToSnakeCase(parameter.baseName);
        if (!parameter.vendorExtensions.containsKey("x-rust-param-identifier")) {
            parameter.vendorExtensions.put("x-rust-param-identifier", inFunctionIdentifier);
        }
    }

    static String parameterScope(CodegenParameter parameter) {
        if (parameter.isPathParam) {
            return "path";
        }
        if (parameter.isQueryParam) {
            return "query";
        }
        if (parameter.isFormParam) {
            return "form";
        }
        if (parameter.isBodyParam) {
            return "body";
        }
        return "param";
    }

    static String parameterToSnakeCase(String name) {
        StringBuilder result = new StringBuilder();
        for (int i = 0; i < name.length(); i++) {
            char c = name.charAt(i);
            if (c == ':') {
              if (result.length() > 0) {
                result.append('_');
              }

              result.append("colon");
            } else if (c >= 65 && c <= 90) {
              if (result.length() > 0) {
                result.append('_');
              }

              result.append(Character.toLowerCase(c));
            } else {
              result.append(c);
            }
        }

        return result.toString();
    }

    @Override
    public OperationsMap postProcessOperationsWithModels(OperationsMap objs, List<ModelMap> allModels) {
        OperationMap objectMap = objs.getOperations();
        List<CodegenOperation> operations = objectMap.getOperation();
        System.err.println("[GEN] postProcModels count=" + operations.size());
        for (CodegenOperation operation : operations) {
            System.err.println("[GEN] opId=" + operation.operationId + " bodyParams=" + (operation.bodyParams==null?"null":operation.bodyParams.size()));
            if (operation.pathParams != null && operation.pathParams.size() > 0) {
                for (CodegenParameter param : operation.allParams) {
                    if (!param.isAnyType || !param.isPathParam && !param.isQueryParam && !param.isHeaderParam) continue;
                    param.dataType = "String";
                    param.isPrimitiveType = true;
                    param.isString = true;
                }
                for (CodegenParameter pathParam : operation.pathParams) {
                    if (!pathParam.baseName.contains("-")) continue;
                    String newName = pathParam.baseName.replace("-", "_");
                    this.LOGGER.info(pathParam.baseName + " cannot be used as a path param. Renamed to " + newName);
                    operation.path = operation.path.replace("{" + pathParam.baseName + "}", "{" + newName + "}");
                    pathParam.baseName = newName;
                }
            }
            for (CodegenParameter param : operation.allParams) {
                if (this.hasUUIDs || !param.isUuid) continue;
                this.hasUUIDs = true;
                break;
            }

            for (CodegenParameter param : operation.allParams) {
                if (!param.isBodyParam) continue;
                if (operation.operationId.equals("add_static_code_scans_by_system_id")) {
                    param.dataType = "Vec<models::StaticCodeRequestPostBody>";
                    param.isContainer = true;
                    operation.vendorExtensions.put("x-static-comment",
                        "// The eMASS spec requires the body to be a SINGLE-ELEMENT ARRAY (see the NOTE\n" +
                        "// above, and emasser `lib/emasse/post.rb` `add`/`clear`, which call\n" +
                        "// `add_static_code_scans_by_system_id(systemId, Array.new(1, body))`). The\n" +
                        "// generated client serializes the argument directly, so it accepts a Vec and\n" +
                        "// serde emits the `[ {...} ]` array. Accepting the Vec (not a single body) is\n" +
                        "// what gives byte-for-byte parity with the reference tool.\n");
                } else if (operation.operationId.equals("add_poam_by_system_id")) {
                    param.dataType = "Vec<serde_json::Value>";
                }
                break;
            }
            if (HYPER_LIBRARY.equals(this.getLibrary())) {
                operation.httpMethod = StringUtils.camelize(operation.httpMethod.toLowerCase(Locale.ROOT));
            } else if (REQWEST_LIBRARY.equals(this.getLibrary()) || REQWEST_TRAIT_LIBRARY.equals(this.getLibrary())) {
                operation.httpMethod = operation.httpMethod.toUpperCase(Locale.ROOT);
            }
            if (!operation.vendorExtensions.containsKey("x-group-parameters") && this.useSingleRequestParameter) {
                operation.vendorExtensions.put("x-group-parameters", Boolean.TRUE);
            }
            if (!operation.producesTextPlain() || !"String".equals(operation.returnType)) continue;
            operation.vendorExtensions.put("x-supports-plain-text", Boolean.TRUE);
        }
        if (!this.hasUUIDs) {
            for (ModelMap map : allModels) {
                CodegenModel m4 = map.getModel();
                if (!m4.getIsUuid() && !this.hasUuidInProperties(m4.vars)) continue;
                this.hasUUIDs = true;
                this.LOGGER.debug("found UUID in model: " + m4.name);
                break;
            }
        }
        this.additionalProperties.put("hasUUIDs", this.hasUUIDs);
        return objs;
    }

    private boolean hasUuidInProperties(List<CodegenProperty> properties) {
        for (CodegenProperty property : properties) {
            if (property.isUuid) {
                return true;
            }
            if (property.items != null && this.hasUuidInProperties(Collections.singletonList(property.items))) {
                return true;
            }
            if (property.additionalProperties != null && this.hasUuidInProperties(Collections.singletonList(property.additionalProperties))) {
                return true;
            }
            if (property.vars == null || !this.hasUuidInProperties(property.vars)) continue;
            return true;
        }
        return false;
    }

    @Override
    public String toDefaultValue(Schema p) {
        if (p.getDefault() != null) {
            return p.getDefault().toString();
        }
        return null;
    }

    @Override
    protected ImmutableMap.Builder<String, Mustache.Lambda> addMustacheLambdas() {
        return super.addMustacheLambdas().put("lifetimeName", new ReplaceAllLambda("^r#", "r_"));
    }

    public static <K, V> Map<V, List<K>> invertMap(Map<K, V> map) {
        HashMap<V, List<K>> invertedMap = new HashMap<V, List<K>>();
        for (Map.Entry<K, V> entry : map.entrySet()) {
            invertedMap.computeIfAbsent(entry.getValue(), k -> new ArrayList<K>()).add(entry.getKey());
        }
        return invertedMap;
    }

    public static <K, V> boolean hasDuplicateValues(Map<K, V> map) {
        HashSet<V> uniqueValues = new HashSet<V>(map.values());
        return uniqueValues.size() < map.size();
    }

    private void setUseSingleRequestParameter(boolean useSingleRequestParameter) {
        this.useSingleRequestParameter = useSingleRequestParameter;
    }

    private void setSupportAsync(boolean supportAsync) {
        this.supportAsync = supportAsync;
    }

    private void setSupportMiddleware(boolean supportMiddleware) {
        this.supportMiddleware = supportMiddleware;
    }

    private void setSupportTokenSource(boolean supportTokenSource) {
        this.supportTokenSource = supportTokenSource;
    }

    public void setPreferUnsignedInt(boolean preferUnsignedInt) {
        this.preferUnsignedInt = preferUnsignedInt;
    }

    public void setBestFitInt(boolean bestFitInt) {
        this.bestFitInt = bestFitInt;
    }

    public void setAvoidBoxedModels(boolean avoidBoxedModels) {
        this.avoidBoxedModels = avoidBoxedModels;
    }

    public void setPackageName(String packageName) {
        this.packageName = packageName;
    }

    public void setPackageVersion(String packageVersion) {
        this.packageVersion = packageVersion;
    }
}

